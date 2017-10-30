#![warn(missing_docs)]
//! The frontend for running the server.
//!
//! Might be replaced with a Hyper-based server one day.
extern crate graph;
extern crate newtypes;
extern crate interface;
extern crate database;
extern crate logic;
extern crate iron;
extern crate mount;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_urlencoded as fromurl;
extern crate serde_json;
#[macro_use]
extern crate log;

use newtypes::Located;
use database::Update;


use std::error::Error;
use iron::Handler;
use iron::{IronResult, Request, Response, BeforeMiddleware};
use mount::Mount;
use interface::Metadata;
use interface::ServingModel;
use interface::Limit;
use std::io;
use std::io::{Write, Read};
use std::sync::Arc;
use std::sync::Mutex;

use std::sync::mpsc::{Sender, channel};

use std::fs;

#[derive(Serialize, Deserialize, Default)]
struct DatabaseInfo {
    url : String,
    username : String,
    password : String,
    schema : String,
}

#[derive(Serialize, Deserialize, Default)]
struct ServerInfo {
    host : String,
    port : u16,
}

#[derive(Serialize, Deserialize, Default)]
struct AlgorithmData {
    rating_influence : f64,

}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    database_config : DatabaseInfo,
    server_info : ServerInfo,
    hyperparameters : AlgorithmData,
}

use std::env;

/// Launch the server.

pub fn fire(config_filename : &str) -> Result<(), Box<Error>>{
    let config = match fs::File::open(config_filename) {
        Ok(file) => serde_json::from_reader(file)?,
        Err(_) => {
            warn!("Failed to open config, creating default config at {}...", config_filename);
            let res = Config::default();
            let _ = writeln!(fs::File::create(config_filename)?, "{}", serde_json::to_string_pretty::<Config>(&res)?);
            res
        }
    };
    let database_config = &config.database_config;
    let database_url = format!("postgresql://{}:{}@{}", database_config.username, env::var("DATABASE_PASSWORD").ok().as_ref().unwrap_or(&database_config.password), database_config.url);
    let graph = logic::get_graph(database::load(&database_url, env::var("SCHEMA").ok().as_ref().unwrap_or(&database_config.schema))?)?;
    let serving_model = logic::ServingModel::get_default_serving_model(graph);
    let serving_model = Arc::new(serving_model);
    let limit = Limit::new(Arc::clone(&serving_model), 0.1);
    let limit = Arc::new(limit);
    let mut mount = Mount::new();
    let sender = async_updater(database_url, env::var("SCHEMA").ok().unwrap_or_else(|| database_config.schema.clone()),  config.hyperparameters.rating_influence);
    mount.mount("/route/generate", GraphHandler::new(Arc::clone(&serving_model), Arc::clone(&limit)));
    mount.mount("/route/return", GraphHandler::new(Arc::clone(&serving_model), Arc::clone(&limit)));
    mount.mount("/route/rate", Rater::new(Arc::clone(&serving_model), sender));
    mount.mount("/route/debug", Debugger::new(Arc::clone(&serving_model)));
    let server_info = &config.server_info;
    let server_location = format!("{}:{}", server_info.host, server_info.port);
    info!("We're up and running!");
    let mut chain = iron::Chain::new(mount);
    chain.link_before(Logger);
    iron::Iron::new(chain).http(&server_location)?;
    Ok(())
}

struct Logger;


impl BeforeMiddleware for Logger {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        info!("Received {:?}", req);
        Ok(())
    }
}

fn async_updater(database_url : String, schema : String, influence : f64) -> Sender<Update> {
    use std::thread;
    let (sx, rx) = channel::<Update>();
    thread::spawn(move ||
        {
            for update in rx {
                println!("{:?}", update.store(&database_url, &schema, influence));
            }
        }
    );
    sx
}

struct GraphHandler {
    serving_model : Arc<ServingModel>,
    limit : Arc<Limit>,
}

impl GraphHandler {
    fn new(serving_model : Arc<ServingModel>, limit : Arc<Limit>) -> GraphHandler {
        GraphHandler {
            serving_model : serving_model,
            limit : limit,
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct RoutingUrlData {
    lon : f64,
    lat : f64,
    visited_path : Option<String>,
    tags : Option<String>,
    neg_tags : Option<String>,
    distance : f64,
    #[serde(rename = "type")]
    type_ : Option<String>
}

impl RoutingUrlData {
    fn get_metadata(&self) -> Result<Metadata, Box<Error>> {
        let mut res = Metadata::default();
        res.requested_length = newtypes::Km::from_f64(self.distance);
        if let Some(ref s) = self.visited_path {
            res.original_route = Some(interface::serialize::to_path(s)?);
        }
        let v = String::new();
        let tag_vec : Vec<_> = self.tags.as_ref().unwrap_or(&v).split('/').collect();
        for tag in &tag_vec {
            let size = 1.0 / tag_vec.len() as f64;
            res.add(tag, size);
        }
        let neg_tag_vec : Vec<_> = self.neg_tags.as_ref().unwrap_or(&v).split('/').collect();
        for tag in &neg_tag_vec {
            let size = 1.0 / neg_tag_vec.len() as f64;
            res.add(tag, -size);
        }
        Ok(res)
    }
}

impl GraphHandler {
    fn handle_loc(&self, parse : RoutingUrlData) -> Result<Response, Box<Error>>  {
        info!("Parsed: {:?}", parse);
        let from = newtypes::Location::new(parse.lon, parse.lat);
        let metadata = parse.get_metadata()?;
        let to = match metadata.original_route {
            None => from.clone(),
            Some(ref path) => match self.serving_model.graph.get(path.last()) {
                None => from.clone(),
                Some(x) => x.located()
            }
        };
        info!("Metadata: {:?}", metadata);
        let path = interface::route(
            &self.serving_model,
            &from,
            &to,
            || metadata.clone(),
            &parse.type_.as_ref().map(|s| interface::RoutingType::from(s))
                .unwrap_or(interface::RoutingType::Directions),
            &self.limit
            )?;

        let response = Response::with((iron::status::Ok, path));
        Ok(response)
    }
}


macro_rules! impl_handler {
    ($type : ty, $data : ty) => {
        impl Handler for $type {
            fn handle(&self, request : &mut Request) -> IronResult<Response> {
                let mut body = String::new();
                request.body.read_to_string(&mut body).map_err(<Box<Error>>::from).and_then(|_|
                    {
                        info!("Parsing {:?}:", body);
                        let parse : Result<$data, _> = fromurl::from_str(&body);
                        parse.map_err(<Box<Error>>::from)
                            .and_then(|parse| self.handle_loc(parse).map_err(<Box<Error>>::from))
                    }).map_err(|e| {
                        error!("{}", e.description());
                        error!("Caused by data dump: {}", body);
                        iron::IronError::new(io::Error::new(io::ErrorKind::Other, e.description().to_string()), (iron::status::NotFound, "Empty route!".to_string()))}
                    )
            }
        }
    }
}
struct Rater {
    serving_model : Arc<ServingModel>,
    sender : Mutex<Sender<Update>>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct RatingData {
    visited_path : String,
    rating : f64,
}

impl Rater {
    pub fn new(serving_model : Arc<ServingModel>, sender : Sender<Update>) -> Rater {
        Rater {
            serving_model : serving_model,
            sender : Mutex::new(sender),
        }
    }

    fn handle_loc(&self, parse : RatingData) -> Result<Response, Box<Error>> {
        let update = interface::rate(&self.serving_model.graph, &interface::serialize::to_path(&parse.visited_path)?, parse.rating);
        {
            self.sender.lock().map_err(|e| e.to_string())?.send(update)?;
        }
        let response = Response::with((iron::status::Ok, "Everything is fine!"));
        Ok(response)
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct DebuggingData {
    password : String,
}

/// For debugging shenanigans
struct Debugger {
    serving_model : Arc<ServingModel>,

}

impl Debugger {
    pub fn new(serving_model : Arc<ServingModel>) -> Debugger {
        Debugger {
            serving_model : serving_model,
        }
    }

    fn handle_loc(&self, parse : DebuggingData) -> Result<Response, Box<Error>> {
        if &parse.password != "Help, I've been transformed into a frog!" {
            Err("Sorry, you're not allowed!")?;
        }
        Ok(Response::with((iron::status::Ok, self.serving_model.debug())))
    }
}

impl_handler!(Rater, RatingData);
impl_handler!(GraphHandler, RoutingUrlData);
impl_handler!(Debugger, DebuggingData);
