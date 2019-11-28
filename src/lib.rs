extern crate rand;
extern crate rand_distr;
extern crate spaces;
extern crate ndarray;
extern crate special_fun;

mod consts;
mod macros;

mod probability;
pub use self::probability::*;

mod distribution;
pub use self::distribution::*;

mod convolution;
pub use self::convolution::*;

pub mod prelude;

pub mod fitting;
pub mod statistics;
pub mod univariate;
pub mod multivariate;

mod mixture;
pub use self::mixture::Mixture;
