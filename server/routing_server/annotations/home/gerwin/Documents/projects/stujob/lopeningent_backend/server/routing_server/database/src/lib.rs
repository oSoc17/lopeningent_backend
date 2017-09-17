               :extern crate postgres;
               :#[macro_use]
               :extern crate database_derive;
               :extern crate newtypes;
               :extern crate graph;
               :
               :use postgres::TlsMode;
               :use postgres::Connection;
               :use std::error::Error;
               :use newtypes::{Located, Location};
               :use graph::{NodeID, EdgeID};
               :
               :pub trait Convert {
               :    type From;
               :    fn convert(from : Self::From) -> Self;
               :}
               :
               :macro_rules! default_impl {
               :    ($($type : ty, $from : ty);*) => {
               :        $(
               :            impl Convert for $type {
               :                type From = $from;
               :                fn convert(from : Self::From) -> Self {
     1 2.6e-05 :                    from as $type
               :                }
               :            }
               :        )*
               :    };
               :}
               :
               :default_impl!(i32, i32; i64, i64; u64, i32; usize, i32; String, String; f32, f32; f64, f64);
               :
               :impl<T : Convert> Convert for Option<T> {
               :    type From = Option<T>;
               :    fn convert(from : Self::From) -> Self {
               :        from
               :    }
               :}
               :
               :impl Convert for Tags {
               :    type From = Vec<String>;
    14 3.7e-04 :    fn convert(t : Vec<String>) -> Tags { /* _$LT$database..Tags$u20$as$u20$database..Convert$GT$::convert::hb731676f0f07c979 total:     42  0.0011 */
               :        let mut res = Tags::default();
     1 2.6e-05 :        for i in t {
               :            match i.as_ref() {
               :                "tourism" => res.tourism = true,
               :                "water" => res.water = true,
               :                "park" => res.park = true,
               :                _ => ()
               :            }
               :        }
     8 2.1e-04 :        res
     3 7.9e-05 :    }
               :}
               :
               :#[derive(Debug, Default)]
               :pub struct Tags {
               :    pub tourism : bool,
               :    pub water : bool,
               :    pub park : bool,
               :}
               :
               :/*impl Convert<Vec<String>> for Tags {
               :    fn convert(t : Vec<String>) -> Tags {
               :        let mut res = Tags::default();
               :        for i in t {
               :            match i.as_ref() {
               :                "tourism" => res.tourism = true,
               :                "water" => res.water = true,
               :                "park" => res.park = true,
               :                _ => ()
               :            }
               :        }
               :        res
               :    }
               :}*/
               :
               :pub trait DebugQuery {
               :    fn debug() -> String;
               :}
               :
               :pub trait Query : Sized {
               :    fn load(conn : &::postgres::Connection) -> Result<Vec<Self>, Box<Error>>;
               :}
               :
    25 6.6e-04 :#[derive(Query, Debug)] /* _$LT$database..Node$u20$as$u20$database..Query$GT$::load::hd238ad772fae8816 total:     27 7.1e-04 */
               :#[table_name = "lopeningent.nodes"]
               :pub struct Node {
               :    pub nid : NodeID,
               :    pub lon : f64,
               :    pub lat : f64,
               :}
               :
               :impl Located for Node {
    20 5.3e-04 :    fn located(&self) -> Location { /* _$LT$database..Node$u20$as$u20$newtypes..newtypes..Located$GT$::located::h7e156d1f0b0ff2b6 total:    248  0.0066 */
   228  0.0060 :        Location::new(self.lon, self.lat)
               :    }
               :}
               :
    59  0.0016 :#[derive(Query, Debug)] /* _$LT$database..Edge$u20$as$u20$database..Query$GT$::load::hd06e7b392ffe6454 total:    214  0.0057 */
               :#[table_name = "lopeningent.edges"]
               :pub struct Edge {
               :    pub eid : EdgeID,
               :    pub rating : f32,
               :    pub tags : Tags,
               :    pub from_node : NodeID,
               :    pub to_node : NodeID,
               :}
               :
     1 2.6e-05 :#[derive(Query, Debug)] /* _$LT$database..Poi$u20$as$u20$database..Query$GT$::load::he421d24b007f3678 total:      2 5.3e-05 */
               :#[table_name = "lopeningent.pois"]
               :pub struct Poi {
               :    pub pid : usize,
               :    pub name : String,
               :    pub description : Option<String>,
               :    pub lon : f64,
               :    pub lat : f64,
               :    pub tag : Option<String>,
               :}
               :
               :impl Located for Poi {
               :    fn located(&self) -> Location {
               :        Location::new(self.lon, self.lat)
               :    }
               :}
               :
               :pub struct Scheme {
               :    pub nodes : Vec<Node>,
               :    pub edges : Vec<Edge>,
               :    pub pois : Vec<Poi>,
               :}
               :
               :
               :
               :pub fn load(database_url : &str) -> Result<Scheme, Box<Error>> {
               :    let connection = Connection::connect(database_url, TlsMode::None)?;
               :    use std::io;
               :    use std::io::Write;
               :    writeln!(io::stderr(), "{}", Node::debug());
               :    writeln!(io::stderr(), "{}", Edge::debug());
               :    writeln!(io::stderr(), "{}", Poi::debug());
               :
               :    Ok(Scheme {
               :        nodes : Node::load(&connection)?,
               :        edges : Edge::load(&connection)?,
               :        pois : Poi::load(&connection)?,
               :    })
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/database/src/lib.rs"
 * 
 *    360  0.0095
 */


/* 
 * Command line: opannotate --source --output-dir=annotations ./target/release/routing_server 
 * 
 * Interpretation of command line:
 * Output annotated source file with samples
 * Output all files
 * 
 * CPU: Intel Ivy Bridge microarchitecture, speed 3100 MHz (estimated)
 * Counted CPU_CLK_UNHALTED events (Clock cycles when not halted) with a unit mask of 0x00 (No unit mask) count 90000
 */
