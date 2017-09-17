               :use fallible_iterator::FallibleIterator;
               :use postgres_protocol::message::backend::DataRowBody;
               :use std::ascii::AsciiExt;
               :use std::io;
               :use std::ops::Range;
               :
               :use stmt::Column;
               :use rows::sealed::Sealed;
               :
               :mod sealed {
               :    use stmt::Column;
               :
               :    pub trait Sealed {
               :        fn __idx(&self, stmt: &[Column]) -> Option<usize>;
               :    }
               :}
               :
               :/// A trait implemented by types that can index into columns of a row.
               :///
               :/// This cannot be implemented outside of this crate.
               :pub trait RowIndex: Sealed {}
               :
               :impl Sealed for usize {
               :    #[inline]
               :    fn __idx(&self, stmt: &[Column]) -> Option<usize> {
     7 1.8e-04 :        if *self >= stmt.len() {
               :            None
               :        } else {
               :            Some(*self)
               :        }
               :    }
               :}
               :
               :impl RowIndex for usize {}
               :
               :impl Sealed for str {
               :    #[inline]
               :    fn __idx(&self, stmt: &[Column]) -> Option<usize> {
               :        if let Some(idx) = stmt.iter().position(|d| d.name() == self) {
               :            return Some(idx);
               :        };
               :
               :        // FIXME ASCII-only case insensitivity isn't really the right thing to
               :        // do. Postgres itself uses a dubious wrapper around tolower and JDBC
               :        // uses the US locale.
               :        stmt.iter().position(
               :            |d| d.name().eq_ignore_ascii_case(self),
               :        )
               :    }
               :}
               :
               :impl RowIndex for str {}
               :
               :impl<'a, T> Sealed for &'a T
               :where
               :    T: ?Sized + Sealed,
               :{
               :    #[inline]
               :    fn __idx(&self, columns: &[Column]) -> Option<usize> {
               :        T::__idx(*self, columns)
               :    }
               :}
               :
               :impl<'a, T> RowIndex for &'a T
               :where
               :    T: ?Sized + Sealed,
               :{
               :}
               :
               :#[doc(hidden)]
               :pub struct RowData {
               :    body: DataRowBody,
               :    ranges: Vec<Option<Range<usize>>>,
               :}
               :
               :impl RowData {
    19 5.0e-04 :    pub fn new(body: DataRowBody) -> io::Result<RowData> { /* postgres_shared::rows::RowData::new::h881f46110d4a0837 total:    254  0.0067 */
     1 2.6e-05 :        let ranges = body.ranges().collect()?;
    12 3.2e-04 :        Ok(RowData {
               :            body: body,
               :            ranges: ranges,
               :        })
     9 2.4e-04 :    }
               :
               :    pub fn len(&self) -> usize {
               :        self.ranges.len()
               :    }
               :
    16 4.2e-04 :    pub fn get(&self, index: usize) -> Option<&[u8]> { /* postgres_shared::rows::RowData::get::hfa92c38fd470cded total:    141  0.0037 */
               :        match &self.ranges[index] {
    40  0.0011 :            &Some(ref range) => Some(&self.body.buffer()[range.clone()]),
               :            &None => None,
               :        }
    13 3.4e-04 :    }
               :}
/* 
 * Total samples for file : "/home/gerwin/.cargo/registry/src/github.com-1ecc6299db9ec823/postgres-shared-0.4.0/src/rows.rs"
 * 
 *    117  0.0031
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
