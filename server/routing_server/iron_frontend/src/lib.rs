#![warn(missing_docs)]


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

use newtypes::Located;
use database::Update;


use std::error::Error;
use iron::Handler;
use iron::{IronResult, Request, Response, BeforeMiddleware};
use mount::Mount;
use interface::Metadata;
use interface::Conversion;
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

pub fn fire(config_filename : &str) -> Result<(), Box<Error>>{
    let config = match fs::File::open(config_filename) {
        Ok(file) => serde_json::from_reader(file)?,
        Err(_) => {
            let _ = writeln!(io::stderr(), "Failed to open config, creating default config at {}...", config_filename);
            let res = Config::default();
            let _ = writeln!(fs::File::create(config_filename)?, "{}", serde_json::to_string_pretty::<Config>(&res)?);
            res
        }
    };
    let database_config = &config.database_config;
    let database_url = format!("postgresql://{}:{}@{}", database_config.username, option_env!("DATABASE_PASSWORD").unwrap_or(&database_config.password), database_config.url);
    let graph = logic::get_graph(database::load(&database_url)?)?;
    let conversion = logic::Conversion::get_default_conversion(graph);
    let conversion = Arc::new(conversion);
    let limit = Limit::new(conversion.clone(), 0.1);
    let limit = Arc::new(limit);
    let mut mount = Mount::new();
    let sender = async_updater(database_url, config.hyperparameters.rating_influence);
    mount.mount("/route/generate", GraphHandler::new(conversion.clone(), limit.clone()));
    mount.mount("/route/return", GraphHandler::new(conversion.clone(), limit.clone()));
    mount.mount("/route/rate", Rater::new(conversion.clone(), sender));
    let server_info = &config.server_info;
    let server_location = format!("{}:{}", server_info.host, server_info.port);
    let _ = writeln!(io::stderr(), "We're up and running!");
    let mut chain = iron::Chain::new(mount);
    chain.link_before(Logger);
    iron::Iron::new(chain).http(&server_location)?;
    Ok(())
}

struct Logger;


impl BeforeMiddleware for Logger {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let _ = writeln!(io::stderr(), "Received {:?}", req);
        Ok(())
    }
}

fn async_updater(database_url : String, influence : f64) -> Sender<Update> {
    use std::thread;
    let (sx, rx) = channel::<Update>();
    thread::spawn(move ||
        {
            for update in rx.into_iter() {
                println!("{:?}", update.store(&database_url, influence));
            }
        }
    );
    sx
}

struct GraphHandler {
    conversion : Arc<Conversion>,
    limit : Arc<Limit>,
}

impl GraphHandler {
    fn new(conversion : Arc<Conversion>, limit : Arc<Limit>) -> GraphHandler {
        GraphHandler {
            conversion : conversion.clone(),
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
            res.original_route = Some(interface::serialize::to_path(&s)?);
        }
        let v = String::new();
        let tag_vec : Vec<_> = self.tags.as_ref().unwrap_or(&v).split("/").collect();
        for tag in &tag_vec {
            let size = 1.0 / tag_vec.len() as f64;
            res.add(tag, size);
        }
        let neg_tag_vec : Vec<_> = self.neg_tags.as_ref().unwrap_or(&v).split("/").collect();
        for tag in &neg_tag_vec {
            let size = 1.0 / neg_tag_vec.len() as f64;
            res.add(tag, -size);
        }
        Ok(res)
    }
}

impl GraphHandler {
    fn handle_loc(&self, parse : RoutingUrlData) -> Result<Response, Box<Error>>  {
        let _ = writeln!(io::stderr(), "Parsed: {:?}", parse);
        let from = newtypes::Location::new(parse.lon, parse.lat);
        let mut metadata = parse.get_metadata()?;
        let to = match metadata.original_route {
            None => from.clone(),
            Some(ref path) => match self.conversion.graph.get(path.last()) {
                None => from.clone(),
                Some(ref x) => x.located()
            }
        };
        let _ = writeln!(io::stderr(), "Metadata: {:?}", metadata);
        let path = interface::route(
            &self.conversion,
            &from,
            &to,
            &mut metadata,
            parse.type_.as_ref().map(|s| interface::RoutingType::from(s))
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
                        let _ = writeln!(io::stderr(), "Parsing {:?}:", body);
                        let parse : Result<$data, _> = fromurl::from_str(&body);
                        parse.map_err(<Box<Error>>::from)
                            .and_then(|parse| self.handle_loc(parse).map_err(<Box<Error>>::from))
                    }).map_err(|e| {
                        println!("{}", e.description());
                        iron::IronError::new(io::Error::new(io::ErrorKind::Other, e.description().to_string()), (iron::status::NotFound, "Empty route!".to_string()))}
                    )
            }
        }
    }
}
struct Rater {
    conversion : Arc<Conversion>,
    sender : Mutex<Sender<Update>>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct RatingData {
    tag : String,
    rating : f64,
}

impl Rater {
    pub fn new(conversion : Arc<Conversion>, sender : Sender<Update>) -> Rater {
        Rater {
            conversion : conversion,
            sender : Mutex::new(sender),
        }
    }

    fn handle_loc(&self, parse : RatingData) -> Result<Response, Box<Error>> {
        let update = interface::rate(&self.conversion.graph, &interface::serialize::to_path(&parse.tag)?, parse.rating);
        {
            self.sender.lock().map_err(|e| e.to_string())?.send(update)?;
        }
        let response = Response::with((iron::status::Ok, "Everything is fine!"));
        Ok(response)
    }
}

impl_handler!(Rater, RatingData);
impl_handler!(GraphHandler, RoutingUrlData);
