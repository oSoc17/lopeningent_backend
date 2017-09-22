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

use newtypes::Located;

use std::error::Error;
use iron::Handler;
use iron::{IronResult, Request, Response};
use mount::Mount;
use logic::Metadata;
use logic::Conversion;
use std::io;
use std::io::{Write, Read};
use std::sync::Arc;

pub fn fire() -> Result<(), Box<Error>>{
    let graph = logic::get_graph(database::load("postgresql://postgres:0987654321@localhost")?)?;
    let conversion = logic::Conversion::get_default_conversion(graph);
    let conversion = Arc::new(conversion);
    let mut mount = Mount::new();
    mount.mount("/route/generate", GraphHandler::new(conversion.clone()));
    mount.mount("/route/return", GraphHandler::new(conversion));
    writeln!(io::stderr(), "We're up and running!");
    iron::Iron::new(mount).http("127.0.0.1:8000")?;
    Ok(())
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
pub struct ParseUrlInto {
    lon : f64,
    lat : f64,
    visited_path : Option<String>,
    tags : Option<String>,
    neg_tags : Option<String>,
    distance : f64,
    #[serde(rename = "type")]
    type_ : Option<String>
}

impl ParseUrlInto {
    fn get_metadata(&self) -> Metadata {
        let mut res = Metadata::default();
        res.requested_length = newtypes::Km::from_f64(self.distance);
        if let Some(ref s) = self.visited_path {
            res.original_route = Some(interface::serialize::to_path(&s));
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
        res
    }
}

impl GraphHandler {
    fn handle_loc(&self, request : &mut Request) -> Result<Response, Box<Error>>  {
        let mut body = String::new();
        request.body.read_to_string(&mut body);
        writeln!(io::stderr(), "Parsing {:?}:", body);
        let parse : ParseUrlInto = fromurl::from_str(&body)?;

        writeln!(io::stderr(), "{:#?}", parse);
        let from = newtypes::Location::new(parse.lon, parse.lat);
        let metadata = parse.get_metadata();
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

impl Handler for GraphHandler {
    fn handle(&self, request : &mut Request) -> IronResult<Response> {
        self.handle_loc(request)
            .map_err(|e| iron::IronError::new(io::Error::new(io::ErrorKind::Other, e.description().to_string()), (iron::status::NotFound, "Empty route!".to_string())))
    }
}
