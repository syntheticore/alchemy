use serde::{Serialize, Deserialize};

use crate::base::*;
use crate::curve::*;

use crate::geom2d;
use crate::geom3d;
use crate::transform::*;
use crate::mesh::*;
// use crate::log;


pub trait Surface: Transformable {
  fn sample(&self, u: f64, v: f64) -> Point3;
  fn unsample(&self, p: Point3) -> (f64, f64);
  fn normal_at(&self, u: f64, v: f64) -> Vec3;
  fn tesselate(&self, resolution: i32, profile: &Profile) -> Mesh;
  fn flip(&mut self); //XXX use Face::flip_normal instead
}

impl core::fmt::Debug for dyn Surface {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "Foo")
  }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurfaceType {
  Planar(Plane),
  Cylindrical(CylindricalSurface),
}

impl SurfaceType {
  pub fn as_surface(&self) -> &dyn Surface {
    match self {
      Self::Planar(plane) => plane,
      Self::Cylindrical(surf) => surf,
    }
  }

  pub fn as_surface_mut(&mut self) -> &mut dyn Surface {
    match self {
      Self::Planar(plane) => plane,
      Self::Cylindrical(surf) => surf,
    }
  }
}


#[derive(Debug)]
pub struct TrimmedSurface {
  pub base: SurfaceType,
  pub bounds: Profile,
}

impl TrimmedSurface {
  pub fn new(surf: SurfaceType, outer_bounds: Wire) -> Self {
    Self {
      base: surf,
      bounds: vec![outer_bounds],
    }
  }

  pub fn area(&self) -> f64 {
    0.0 //XXX
  }

  //XXX pub fn on_surface(&self, u: f64, v: f64) -> bool
}

impl Meshable for TrimmedSurface {
  fn tesselate(&self) -> Mesh {
    self.base.as_surface().tesselate(80, &self.bounds)
  }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plane {
  pub origin: Point3,
  pub u: Vec3,
  pub v: Vec3,
}

impl Default for Plane {
  fn default() -> Self {
    Self::new()
  }
}

impl Plane {
  pub fn new() -> Self {
    Self {
      origin: Point3::origin(),
      u: Vec3::new(1.0, 0.0, 0.0),
      v: Vec3::new(0.0, 1.0, 0.0),
    }
  }

  pub fn from_point(p: Point3) -> Self {
    Self {
      origin: p,
      u: Vec3::new(1.0, 0.0, 0.0),
      v: Vec3::new(0.0, 1.0, 0.0),
    }
  }

  pub fn from_triangle(p1: Point3, p2: Point3, p3: Point3) -> Self {
    let u = (p2 - p1).normalize();
    let pre_v = (p3 - p1).normalize();
    let normal = u.cross(pre_v).normalize();
    Self {
      origin: p1,
      u,
      v: u.cross(normal),
    }
  }

  pub fn from_normal(origin: Point3, normal: Vec3) -> Self {
    let m = geom3d::transform_from_location_and_normal(origin, normal);
    Self {
      origin,
      u: m.transform_vector(Vec3::new(1.0, 0.0, 0.0)),
      v: m.transform_vector(Vec3::new(0.0, 1.0, 0.0)),
    }
  }

  pub fn normal(&self) -> Vec3 {
    self.u.cross(self.v)
  }

  pub fn contains_point(&self, p: Point3) -> bool {
    self.origin.almost(p) ||
    self.normal().dot((p - self.origin).normalize()).abs().almost(0.0)
  }

  // https://github.com/xibyte/jsketcher/blob/master/modules/geom/euclidean.ts
  // export function perpendicularVector(v) {
  //   v = vec.normalize(v);
  //   return IDENTITY_BASIS3.map(axis => vec.cross(axis, v)).sort((a, b) => vec.lengthSq(b) - vec.lengthSq(a))[0];
  // }

  pub fn as_transform(&self) -> Matrix4 {
    Matrix4::from_cols(
      self.u.extend(0.0),
      self.v.extend(0.0),
      self.normal().extend(0.0),
      self.origin.to_vec().extend(1.0)
    )
  }

  pub fn into_enum(self) -> SurfaceType {
    SurfaceType::Planar(self)
  }
}

impl Surface for Plane {
  fn sample(&self, u: f64, v: f64) -> Point3 {
    self.origin + self.u * u + self.v * v
  }

  fn unsample(&self, p: Point3) -> (f64, f64) {
    let p_local = self.as_transform().invert().unwrap().transform_point(p);
    (p_local.x, p_local.y)
  }

  fn normal_at(&self, _u: f64, _v: f64) -> Vec3 {
    self.normal()
  }

  fn tesselate(&self, _resolution: i32, profile: &Profile) -> Mesh {
    let mut local_profile = profile.clone();
    let trans = self.as_transform();
    let trans_inv = trans.invert().unwrap();
    for wire in &mut local_profile {
      for curve in wire.iter_mut() {
        curve.transform(&trans_inv);
      }
    }
    let mut mesh = geom2d::tesselate_profile(&local_profile, self.normal());
    mesh.transform(&trans);
    mesh
  }

  fn flip(&mut self) {
    self.v = -self.v;
  }
}

impl Transformable for Plane {
  fn transform(&mut self, transform: &Matrix4) {
    self.origin = transform.transform_point(self.origin);
    self.u = transform.transform_vector(self.u);
    self.v = transform.transform_vector(self.v);
  }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CylindricalSurface {
  pub origin: Point3,
  pub radius: f64,
  pub direction: Vec3,
  pub bounds: (f64, f64),
}

impl CylindricalSurface {
  pub fn new(radius: f64) -> Self {
    Self {
      origin: Point3::origin(),
      direction: Vec3::new(0.0, 0.0, 1.0),
      radius,
      bounds: (0.0, 1.0),
    }
  }

  pub fn into_enum(self) -> SurfaceType {
    SurfaceType::Cylindrical(self)
  }
}

impl Surface for CylindricalSurface {
  fn sample(&self, u: f64, v: f64) -> Point3 {
    let u = self.bounds.0 + u * (self.bounds.1 - self.bounds.0);
    let u = u * std::f64::consts::PI * 2.0;
    Point3::new(
      self.origin.x + u.sin() * self.radius,
      self.origin.y + u.cos() * self.radius,
      self.origin.z + self.direction.z * v,
    )
  }

  fn unsample(&self, _p: Point3) -> (f64, f64) {
    todo!()
  }

  fn normal_at(&self, u: f64, _v: f64) -> Vec3 {
    (self.sample(u, 0.0) - self.origin).normalize()
  }

  fn tesselate(&self, resolution: i32, _profile: &Profile) -> Mesh {
    let mut vertices: Vec<Point3> = vec![];
    let mut faces: Vec<usize> = vec![];
    let mut normals: Vec<Vec3> = vec![];
    let mut iter = (0..=resolution).peekable();
    while let Some(i) = iter.next() {
      let u = i as f64 / resolution as f64;
      let upper_left = self.sample(u, 1.0);
      let lower_left = self.sample(u, 0.0);
      vertices.push(lower_left);
      vertices.push(upper_left);
      let normal = self.normal_at(u, 0.0);
      if let Some(&next_i) = iter.peek() {
        let next_u = next_i as f64 / resolution as f64;
        let next_normal = self.normal_at(next_u, 0.0);
        let i = i as usize * 2;
        // Triangle
        faces.push(i);
        faces.push(i + 1);
        faces.push(i + 2);
        normals.push(normal);
        normals.push(normal);
        normals.push(next_normal);
        // Triangle
        faces.push(i + 1);
        faces.push(i + 3);
        faces.push(i + 2);
        normals.push(normal);
        normals.push(next_normal);
        normals.push(next_normal);
      }
    }
    Mesh {
      vertices,
      faces,
      normals,
    }
    // Mesh::default()
  }

  fn flip(&mut self) {
    self.bounds = (self.bounds.1, self.bounds.0);
  }
}

impl Transformable for CylindricalSurface {
  fn transform(&mut self, transform: &Matrix4) {
    self.origin = transform.transform_point(self.origin);
    self.direction = transform.transform_vector(self.direction);
  }
}


// EllipticalSurface
// ConicSurface
// EllipticalConicSurface
// SphericalSurface
// ToricSurface
// NurbsSurface


#[cfg(test)]
mod tests {
  use super::*;
  // use crate::test_data::make_generic;
  // use crate::test_data::make_region;

  #[test]
  fn plane_normal() {
    let mut plane = Plane::new();
    assert_eq!(plane.normal(), Vec3::new(0.0, 0.0, 1.0));
    plane.flip();
    assert_eq!(plane.normal(), Vec3::new(0.0, 0.0, -1.0));
  }

  #[test]
  fn cylinder_normal() {
    let cylinder = CylindricalSurface::new(1.0);
    almost_eq(cylinder.normal_at(0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    almost_eq(cylinder.normal_at(0.5, 0.0), Vec3::new(0.0, -1.0, 0.0));
    almost_eq(cylinder.normal_at(0.25, 0.0), Vec3::new(1.0, 0.0, 0.0));
    almost_eq(cylinder.normal_at(0.75, 0.0), Vec3::new(-1.0, 0.0, 0.0));
  }

  #[test]
  fn plane_transform1() {
    let p = Point3::new(0.0, 0.0, 20.0);
    let plane = Plane {
      origin: Point3::new(-7.071067811865475, 7.0710678118654755, 0.0),
      u: Vec3::new(0.0, 0.0, 1.0),
      v: Vec3::new(-0.7071067811865475, 0.7071067811865476, 0.0),
    };
    let trans = plane.as_transform();
    let p = trans.transform_point(p);
    almost_eq(p.z, 0.0);
  }

  #[test]
  fn plane_transform2() {
    let dist = 6.0;
    let vec = Vec3::new(0.4, 0.5, 1.0).normalize() * dist;
    println!("input vector {:#?}", vec);
    let plane = Plane::from_normal(Point3::new(1.0, 2.0, 3.0), vec.normalize());
    let normal = plane.normal() * dist;
    println!("normal {:#?}", normal);
    let gen_normal = plane.as_transform().transform_vector(Vec3::new(0.0, 0.0, dist));
    println!("generated normal {:#?}", gen_normal);
    almost_eq(vec, normal);
    almost_eq(normal, gen_normal);
  }
}
