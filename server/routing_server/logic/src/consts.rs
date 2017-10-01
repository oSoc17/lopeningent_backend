
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
    min : 0.5,
    max : 0.8,
    increase : 0.04,
    min_lin : 400.0,
    max_lin : 500.0
};

pub const MIN_LENGTH_FACTOR : f64 = 0.8;
pub const DILUTE_FAVOURITE : f64 = 1.5;
pub const FALLOFF : f64 = 4.0;
pub const ABS_MINIMUM : f64 = 0.25;
pub const ABS_MAXIMUM : f64 = 4.0;
