
pub const EARTH_RADIUS : f64 = 6731.0;
pub const BIN_SIZE : f64 = 1.0;
pub const TOLERANCE : f64 = 0.1;
pub struct RandomConfig {
    pub min : f64,
    pub max : f64,
    pub increase : f64,
    pub min_lin : f64,
    pub max_lin : f64,
}

pub const CONFIG : RandomConfig = RandomConfig {
    min : 0.6,
    max : 1.2,
    increase : 0.08,
    min_lin : 400.0,
    max_lin : 700.0
};

pub const MIN_LENGTH_FACTOR : f64 = 0.8;
pub const DILUTE_FAVOURITE : f64 = 1.0;
pub const FALLOFF : f64 = 2.0;
pub const ABS_MINIMUM : f64 = 0.2;
pub const ABS_MAXIMUM : f64 = 5.0;
pub const EVENT_IMPORTANCE : f64 = 2.0;
