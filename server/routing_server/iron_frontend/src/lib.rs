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
use iron::{IronResult, Request, Response};
use mount::Mount;
use logic::Metadata;
use logic::Conversion;
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
        Err(e) => {
            writeln!(io::stderr(), "Failed to open config, creating default config at {}...", config_filename);
            let res = Config::default();
            writeln!(fs::File::create(config_filename)?, "{}", serde_json::to_string_pretty::<Config>(&res)?);
            res
        }
    };
    let database_config = &config.database_config;
    let database_url = format!("postgresql://{}:{}@{}", database_config.username, database_config.password, database_config.url);
    let graph = logic::get_graph(database::load(&database_url)?)?;
    let conversion = logic::Conversion::get_default_conversion(graph);
    let conversion = Arc::new(conversion);
    let mut mount = Mount::new();
    let sender = async_updater(database_url, config.hyperparameters.rating_influence);
    mount.mount("/route/generate", GraphHandler::new(conversion.clone()));
    mount.mount("/route/return", GraphHandler::new(conversion.clone()));
    mount.mount("/route/rate", Rater::new(conversion.clone(), sender));
    let server_info = &config.server_info;
    let server_location = format!("{}:{}", server_info.host, server_info.port);
    writeln!(io::stderr(), "We're up and running!");
    iron::Iron::new(mount).http(&server_location)?;
    Ok(())
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
}

impl GraphHandler {
    fn new(conversion : Arc<Conversion>) -> GraphHandler {
        GraphHandler {
            conversion : conversion,
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
    fn handle_loc(&self, request : &mut Request) -> Result<Response, Box<Error>>  {
        let mut body = String::new();
        request.body.read_to_string(&mut body);
        writeln!(io::stderr(), "Parsing {:?}:", body);
        let parse : RoutingUrlData = fromurl::from_str(&body)?;

        writeln!(io::stderr(), "{:#?}", parse);
        let from = newtypes::Location::new(parse.lon, parse.lat);
        let metadata = parse.get_metadata()?;
        let to = match metadata.original_route {
            None => from.clone(),
            Some(ref path) => match self.conversion.graph.get(path.last()) {
                None => from.clone(),
                Some(ref x) => x.located()
            }
        };
        writeln!(io::stderr(), "{:#?}", metadata);
        let path = interface::route(
            &self.conversion,
            &from,
            &to,
            &metadata,
            parse.type_.as_ref().map(|s| interface::RoutingType::from(s)).unwrap_or(interface::RoutingType::Directions)
            )?;

        let response = Response::with((iron::status::Ok, path));
        Ok(response)
    }
}


macro_rules! impl_handler {
    ($type : ty) => {
        impl Handler for $type {
            fn handle(&self, request : &mut Request) -> IronResult<Response> {
                self.handle_loc(request)
                    .map_err(|e| {
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
}

impl Rater {
    fn handle_loc(&self, request : &mut Request) -> Result<Response, Box<Error>> {
        let mut body = String::new();
        request.body.read_to_string(&mut body);
        let parse : RatingData = fromurl::from_str(&body)?;
        let update = interface::rate(&self.conversion.graph, &interface::serialize::to_path(&parse.tag)?, parse.rating);
        {
            self.sender.lock().map_err(|e| e.to_string())?.send(update);
        }
        let response = Response::with((iron::status::Ok, "Everything is fine!"));
        Ok(response)
    }
}

impl_handler!(Rater);
impl_handler!(GraphHandler);
