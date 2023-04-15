pub mod equivalance;
pub mod svg;

mod img;
mod partition;
mod polygon;

pub use crate::utility::{Color, Rect, TwoDimIter, V};

pub use img::Image;
pub use partition::Partition;
pub use polygon::Polygon;
