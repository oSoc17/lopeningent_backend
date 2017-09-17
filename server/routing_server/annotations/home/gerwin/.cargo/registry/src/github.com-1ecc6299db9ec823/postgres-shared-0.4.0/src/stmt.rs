               :use types::Type;
               :
               :/// Information about a column of a Postgres query.
               :#[derive(Debug)]
               :pub struct Column {
               :    name: String,
               :    type_: Type,
               :}
               :
               :impl Column {
               :    #[doc(hidden)]
               :    pub fn new(name: String, type_: Type) -> Column {
               :        Column {
               :            name: name,
               :            type_: type_,
               :        }
               :    }
               :
               :    /// Returns the name of the column.
               :    pub fn name(&self) -> &str {
               :        &self.name
               :    }
               :
               :    /// Returns the type of the column.
    14 3.7e-04 :    pub fn type_(&self) -> &Type { /* postgres_shared::stmt::Column::type_::heff2674498a0a895 total:     24 6.3e-04 */
     5 1.3e-04 :        &self.type_
     5 1.3e-04 :    }
               :}
/* 
 * Total samples for file : "/home/gerwin/.cargo/registry/src/github.com-1ecc6299db9ec823/postgres-shared-0.4.0/src/stmt.rs"
 * 
 *     24 6.3e-04
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
