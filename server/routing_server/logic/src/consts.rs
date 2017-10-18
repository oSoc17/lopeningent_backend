/// List of constants.
///
/// Constants are divided in two groups: the "Play with them"-group and the "DO NOT TOUCH"-group.


mod fixed {
    /// Earth radius
    pub const EARTH_RADIUS : f64 = 6731.0;
    /// Grid bin size.
    pub const BIN_SIZE : f64 = 1.0;
    /// Interval tolerance.
    pub const TOLERANCE : f64 = 0.1;
}
pub use self::fixed::*;

mod hyperparams {
    /// Structure for holding the randomized parameters.
    pub struct RandomConfig {
        /// Minimal poisoning size
        pub min : f64,
        /// Maximal poisoning size
        pub max : f64,
        /// Difference between the two poison sizes
        pub increase : f64,
        /// Minimal treshold
        pub min_lin : f64,
        /// Maximal treshold
        pub max_lin : f64,
    }

    /// Random parameter bounds.
    pub const CONFIG : RandomConfig = RandomConfig {
        min : 0.6,
        max : 1.2,
        increase : 0.08,
        min_lin : 400.0,
        max_lin : 700.0
    };

    // Mini
    /// Ratio between minimal and expected length
    pub const MIN_LENGTH_FACTOR : f64 = 0.8;

    // Potential function
    /// Potential function peak when hitting a tag.
    pub const DILUTE_FAVOURITE : f64 = 0.5;
    /// Potential function derivate after hitting a tag.
    pub const FALLOFF : f64 = 0.5;
    /// Minimum of the potential function
    pub const ABS_MINIMUM : f64 = 0.5;
    /// Maximum of the potential function
    pub const ABS_MAXIMUM : f64 = 2.0;

    // Meta choice
    /// Strength of events on the route choice after tree generation.
    pub const EVENT_IMPORTANCE : f64 = 2.0;
}

pub use self::hyperparams::*;
