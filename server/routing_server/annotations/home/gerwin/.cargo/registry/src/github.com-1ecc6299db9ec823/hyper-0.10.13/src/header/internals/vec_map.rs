               :#[derive(Clone)]
               :pub struct VecMap<K, V> {
               :    vec: Vec<(K, V)>,
               :}
               :
               :impl<K: PartialEq, V> VecMap<K, V> {
               :    pub fn new() -> VecMap<K, V> {
               :        VecMap {
               :            vec: Vec::new()
               :        }
               :    }
               :
               :    pub fn insert(&mut self, key: K, value: V) {
               :        match self.find(&key) {
               :            Some(pos) => self.vec[pos] = (key, value),
               :            None => self.vec.push((key, value))
               :        }
               :    }
               :
               :    pub fn entry(&mut self, key: K) -> Entry<K, V> {
               :        match self.find(&key) {
               :            Some(pos) => Entry::Occupied(OccupiedEntry {
               :                vec: self,
               :                pos: pos,
               :            }),
               :            None => Entry::Vacant(VacantEntry {
               :                vec: self,
               :                key: key,
               :            })
               :        }
               :    }
               :
               :    pub fn get(&self, key: &K) -> Option<&V> {
               :        self.find(key).map(move |pos| &self.vec[pos].1)
               :    }
               :
               :    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
               :        self.find(key).map(move |pos| &mut self.vec[pos].1)
               :    }
               :
               :    pub fn contains_key(&self, key: &K) -> bool {
               :        self.find(key).is_some()
               :    }
               :
               :    pub fn len(&self) -> usize { self.vec.len() }
               :    pub fn iter(&self) -> ::std::slice::Iter<(K, V)> {
               :        self.vec.iter()
               :    }
               :    pub fn remove(&mut self, key: &K) -> Option<V> {
               :        self.find(key).map(|pos| self.vec.remove(pos)).map(|(_, v)| v)
               :    }
               :    pub fn clear(&mut self) {
               :        self.vec.clear();
               :    }
               :
     1 2.6e-05 :    fn find(&self, key: &K) -> Option<usize> { /* _$LT$hyper..header..internals..vec_map..VecMap$LT$K$C$$u20$V$GT$$GT$::find::h04ea86a15eaa64e0      1 2.6e-05, _$LT$hyper..header..internals..vec_map..VecMap$LT$K$C$$u20$V$GT$$GT$::find::h8a94c66d572f57f0      2 5.3e-05, _$LT$hyper..header..internals..vec_map..VecMap$LT$K$C$$u20$V$GT$$GT$::find::h4223bf5f25007d82      1 2.6e-05, total:      4 1.1e-04 */
               :        self.vec.iter().position(|entry| key == &entry.0)
               :    }
               :}
               :
               :pub enum Entry<'a, K: 'a, V: 'a> {
               :    Vacant(VacantEntry<'a, K, V>),
               :    Occupied(OccupiedEntry<'a, K, V>)
               :}
               :
               :pub struct VacantEntry<'a, K: 'a, V: 'a> {
               :    vec: &'a mut VecMap<K, V>,
               :    key: K,
               :}
               :
               :impl<'a, K, V> VacantEntry<'a, K, V> {
               :    pub fn insert(self, val: V) -> &'a mut V {
               :        let vec = self.vec;
               :        vec.vec.push((self.key, val));
               :        let pos = vec.vec.len() - 1;
               :        &mut vec.vec[pos].1
               :    }
               :}
               :
               :pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
               :    vec: &'a mut VecMap<K, V>,
               :    pos: usize,
               :}
               :
               :impl<'a, K, V> OccupiedEntry<'a, K, V> {
               :    pub fn into_mut(self) -> &'a mut V {
               :        &mut self.vec.vec[self.pos].1
               :    }
               :}
/* 
 * Total samples for file : "/home/gerwin/.cargo/registry/src/github.com-1ecc6299db9ec823/hyper-0.10.13/src/header/internals/vec_map.rs"
 * 
 *      1 2.6e-05
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
