use uuid::Uuid;

// pub use cgmath::prelude::*;
pub use cgmath::prelude::Matrix;
pub use cgmath::prelude::SquareMatrix;
pub use cgmath::prelude::InnerSpace;
pub use cgmath::prelude::MetricSpace;
pub use cgmath::prelude::EuclideanSpace;
pub use cgmath::Transform;
pub use cgmath::Rad;
pub use cgmath::Deg;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub use crate::geom3d::Axis;
pub use crate::geom3d::Plane;

// pub const MAX_FLOAT: f64 = 2_i32.pow(63).into();
pub const MAX_FLOAT: f64 = 9223372036854776000.0;
pub const EPSILON: f64 = core::f64::EPSILON * 1000000.0;

pub type Vec2 = cgmath::Vector2<f64>;
pub type Vec3 = cgmath::Vector3<f64>;
pub type Vec4 = cgmath::Vector4<f64>;
pub type Point2 = cgmath::Point2<f64>;
pub type Point3 = cgmath::Point3<f64>;
pub type Matrix3 = cgmath::Matrix3<f64>;
pub type Matrix4 = cgmath::Matrix4<f64>;
pub type Quaternion = cgmath::Quaternion<f64>;

pub trait Identity {
  fn id(&self) -> Uuid;
}

pub trait Almost {
  // const EPS: f64 = core::f64::EPSILON * 100000.0;
  fn almost(&self, other: Self) -> bool;
}

impl Almost for Point3 {
  fn almost(&self, other: Self) -> bool {
    (self.x - other.x).abs() <= EPSILON &&
    (self.y - other.y).abs() <= EPSILON &&
    (self.z - other.z).abs() <= EPSILON
  }
}

impl Almost for Vec3 {
  fn almost(&self, other: Self) -> bool {
    (self.x - other.x).abs() <= EPSILON &&
    (self.y - other.y).abs() <= EPSILON &&
    (self.z - other.z).abs() <= EPSILON
  }
}

impl Almost for f64 {
  fn almost(&self, other: Self) -> bool {
    (self - other).abs() <= EPSILON
  }
}
