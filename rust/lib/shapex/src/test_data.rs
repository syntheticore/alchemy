use crate::base::*;
use crate::curve::*;

pub fn crossing_lines() -> (Line, Line) {
  (
    Line::new(Point3::new(-0.5, 0.0, 0.0), Point3::new(0.5, 0.0, 0.0)),
    Line::new(Point3::new(0.0, 0.0, -0.5), Point3::new(0.0, 0.0, 0.5)),
  )
}

pub fn parallel_lines() -> (Line, Line) {
  (
    Line::new(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)),
    Line::new(Point3::new(0.0, 0.0, 0.5), Point3::new(1.0, 0.0, 0.5)),
  )
}

pub fn overlapping_lines() -> (Line, Line) {
  (
    Line::new(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)),
    Line::new(Point3::new(0.5, 0.0, 0.0), Point3::new(1.5, 0.0, 0.0)),
  )
}

pub fn t_section() -> (Line, Line) {
  (
    Line::new(Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)),
    Line::new(Point3::new(0.0, 0.0, 1.0), Point3::new(0.0, 0.0, 0.0)),
  )
}

pub fn rectangle() -> (Line, Line, Line, Line) {
  let upper_left = Point3::new(-1.0, 0.0, 1.0);
  let upper_right = Point3::new(1.0, 0.0, 1.0);
  let lower_right = Point3::new(1.0, 0.0, -1.0);
  let lower_left = Point3::new(-1.0, 0.0, -1.0);
  (
    Line::new(upper_left, upper_right),
    Line::new(upper_right, lower_right),
    Line::new(lower_right, lower_left),
    Line::new(lower_left, upper_left),
  )
}

pub fn crossing_rectangle() -> (Line, Line, Line, Line) {
  let displacement = Vec3::new(0.5, 0.0, 0.5);
  let mut rect = rectangle();
  // Displace lower line
  rect.2.points.0 = rect.2.points.0 + displacement;
  rect.2.points.1 = rect.2.points.1 + displacement;
  // Displace left line
  rect.3.points.0 = rect.3.points.0 + displacement;
  rect.3.points.1 = rect.3.points.1 + displacement;
  rect
}