use std::ptr;
use std::rc::Rc;
use std::collections::HashSet;

use uuid::Uuid;

use shapex::*;

pub mod io;


#[macro_export] macro_rules! log {
  ( $( $t:tt )* ) => {
    web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}


#[derive(Debug, Default)]
pub struct Component {
  pub id: Uuid,
  pub sketch: Sketch,
  pub compound: Compound,
  pub children: Vec<Ref<Self>>,
}

impl Component {
  pub fn new() -> Self {
    let mut this: Self = Default::default();
    this.id = Uuid::new_v4();
    this
  }

  pub fn create_component(&mut self) -> Ref<Self> {
    let comp = rc(Self::new());
    self.children.push(comp.clone());
    comp
  }

  pub fn delete_component(&mut self, comp: &Ref<Self>) {
    self.children.retain(|child| !Rc::ptr_eq(child, comp) )
  }
}


#[derive(Debug)]
pub struct Sketch {
  pub elements: Vec<Ref<CurveType>>,
  pub work_plane: Matrix4,
}

impl Default for Sketch {
  fn default() -> Self {
    Self {
      elements: vec![],
      work_plane: Matrix4::one(),
    }
  }
}

impl Sketch {
  pub fn get_profiles(&self, include_outer: bool) -> Vec<Profile> {
    let planar_elements = self.get_planarized_elements();
    let cut_elements = Self::all_split(&planar_elements);
    let regions = Self::get_regions(cut_elements, include_outer);
    let mut profiles = Self::build_profiles(regions);
    // Transform generated profiles back to component space
    for profile in &mut profiles {
      for wire in profile {
        for tcurve in wire {
          tcurve.transform(&self.work_plane);
        }
      }
    }
    profiles
  }

  pub fn get_planarized_elements(&self) -> Vec<Ref<CurveType>> {
    let transform = self.work_plane.invert().unwrap();
    self.elements.iter()
    // Tranform elements to work plane
    .map(|elem| {
      let mut clone = elem.borrow().clone();
      clone.as_curve_mut().transform(&transform);
      rc(clone)
    })
    // Filter elements directly on work plane
    .filter(|elem| {
      let endpoints = tuple2_to_vec(elem.borrow().as_curve().endpoints());
      endpoints.iter().all(|p| p.z.almost(0.0) )
    }).collect()
  }

  fn build_profiles(regions: Vec<Wire>) -> Vec<Profile> {
    regions.iter().map(|wire| {
      // Find all other wires enclosed by this one
      let mut cutouts = vec![];
      for other in &regions {
        if ptr::eq(&*wire, &*other) { continue }
        if geom2d::wire_in_wire(other, wire) {
          cutouts.push(other.clone());
        }
      }
      let mut profile = vec![wire.clone()];
      // Only leave the outermost inner wires
      profile.append(&mut cutouts.iter().filter(|cutout|
        !cutouts.iter().any(|other|
          !ptr::eq(&cutout[0], &other[0]) && geom2d::wire_in_wire(cutout, other)
        )
      ).cloned().collect());
      profile
    }).collect()
  }

  fn get_regions(cut_elements: Vec<TrimmedCurve>, include_outer: bool) -> Vec<Wire> {
    let (circles, mut others) = cut_elements.into_iter().partition(|elem| match elem.base {
      CurveType::Circle(_) => true,
      _ => false,
    });
    Self::remove_dangling_segments(&mut others);
    let islands = Self::build_islands(&others);
    let mut regions: Vec<Wire> = islands.iter()
    .flat_map(|island| Self::build_loops_from_island(island, include_outer) ).collect();
    let mut circle_regions = circles
    .into_iter().map(|circle| vec![circle] ).collect();
    regions.append(&mut circle_regions);
    regions
  }

  fn build_loops_from_island(island: &Vec<TrimmedCurve>, include_outer: bool) -> Vec<Wire> {
    let mut regions = vec![];
    let mut used_forward = HashSet::new();
    let mut used_backward = HashSet::new();
    for start_elem in island.iter() {
      let points = tuple2_to_vec(start_elem.bounds);
      for i in 0..2 {
        let mut loops = Self::build_loop(
          &points[i],
          &start_elem,
          vec![],
          island,
          &mut used_forward,
          &mut used_backward,
        );
        for region in &mut loops { geom2d::straighten_bounds(region) }
        regions.append(&mut loops);
      }
    }
    if !include_outer { Self::remove_outer_loop(&mut regions) }
    regions
  }

  pub fn all_split(elements: &Vec<Ref<CurveType>>) -> Vec<TrimmedCurve> {
    elements.iter().flat_map(|elem| {
      let splits = Self::split_element(&elem.borrow(), &elements);
      splits.into_iter().map(|split| TrimmedCurve {
        base: (*elem.borrow()).clone(),
        bounds: split.as_curve().endpoints(),
        cache: split,
      }).collect::<Vec<TrimmedCurve>>()
    }).collect()
  }

  pub fn split_element(elem: &CurveType, others: &Vec<Ref<CurveType>>) -> Vec<CurveType> {
    let others = others.iter().map(|other| other.borrow().clone() ).collect();
    elem.split_multi(&others)
  }

  pub fn build_islands(elements: &Vec<TrimmedCurve>) -> Vec<Vec<TrimmedCurve>> {
    let mut unused_elements = elements.clone();
    let mut islands = vec![];
    while let Some(start_elem) = unused_elements.pop() {
      let mut island = vec![];
      Self::build_island(&start_elem, &mut island, &unused_elements);
      for island_elem in island.iter() {
        unused_elements.retain(|elem| elem.bounds != island_elem.bounds);
      }
      if island.len() > 0 { islands.push(island) }
    }
    islands
  }

  fn build_island(start_elem: &TrimmedCurve, mut path: &mut Vec<TrimmedCurve>, all_elements: &Vec<TrimmedCurve>) {
    if path.iter().any(|e| e == start_elem ) { return }
    let (start_point, end_point) = start_elem.bounds;
    path.push(start_elem.clone());
    for elem in all_elements.iter() {
      let (other_start, other_end) = elem.bounds;
      // We are connected to other element
      if end_point.almost(other_start) ||
         end_point.almost(other_end) ||
         start_point.almost(other_start) ||
         start_point.almost(other_end)
      {
        Self::build_island(&elem, &mut path, all_elements);
      }
    }
  }

  // https://stackoverflow.com/questions/838076/small-cycle-finding-in-a-planar-graph
  fn build_loop<'a>(
    start_point: &Point3,
    start_elem: &'a TrimmedCurve,
    mut path: Region,
    all_elements: &'a Vec<TrimmedCurve>,
    used_forward: &mut HashSet<Uuid>,
    used_backward: &mut HashSet<Uuid>,
  ) -> Vec<Region> {
    let mut regions = vec![];
    // Traverse edges only once in every direction
    let start_elem_id = as_id(&start_elem.cache).id();
    if start_point.almost(start_elem.bounds.0) {
      if used_forward.contains(&start_elem_id) { return regions }
      used_forward.insert(start_elem_id);
    } else {
      if used_backward.contains(&start_elem_id) { return regions }
      used_backward.insert(start_elem_id);
    }
    // Add start_elem to path
    path.push(start_elem.clone());
    // Find connected segments
    let end_point = start_elem.other_bound(&start_point);
    let mut connected_elems: Vec<&TrimmedCurve> = all_elements.iter().filter(|other_elem| {
      let (other_start, other_end) = other_elem.bounds;
      (end_point.almost(other_start) || end_point.almost(other_end)) &&
        as_id(&other_elem.cache).id() != start_elem_id
    }).collect();
    if connected_elems.len() > 0 {
      // Sort connected segments in clockwise order
      connected_elems.sort_by(|a, b| { //XXX min_by_key
        let final_point_a = a.other_bound(&end_point);
        let final_point_b = b.other_bound(&end_point);
        geom2d::clockwise(*start_point, end_point, final_point_b).partial_cmp(
          &geom2d::clockwise(*start_point, end_point, final_point_a)
        ).unwrap()
      });
      // Follow the leftmost segment to complete loop in anti-clockwise order
      let next_elem = connected_elems[0];
      if as_id(&path[0].cache).id() == as_id(&next_elem.cache).id() {
        // We are closing a loop
        regions.push(path);
      } else {
        // Follow loop
        let mut new_regions = Self::build_loop(
          &end_point,
          &next_elem,
          path,
          all_elements,
          used_forward,
          used_backward
        );
        regions.append(&mut new_regions);
      }
    }
    regions
  }

  fn remove_outer_loop(loops: &mut Vec<Wire>) {
    if loops.len() <= 1 { return }
    loops.retain(|region| {
      !geom2d::is_clockwise(&geom2d::poly_from_wire(region))
    })
  }

  pub fn remove_dangling_segments(island: &mut Vec<TrimmedCurve>) {
    let others = island.clone();
    let start_len = island.len();
    island.retain(|elem| {
      if elem.cache.as_curve().length().almost(0.0) { return false }
      let (start_point, end_point) = elem.bounds;
      // Keep closed circles, arcs and splines
      if start_point == end_point { return true }
      [start_point, end_point].iter().all(|endpoint| {
        others.iter().any(|other_elem| {
          let (other_start, other_end) = other_elem.bounds;
          (endpoint.almost(other_start) || endpoint.almost(other_end))
          && other_elem.bounds != elem.bounds
        })
      })
    });
    if island.len() < start_len { Self::remove_dangling_segments(island) }
  }
}


pub fn as_id(elem: &CurveType) -> &dyn Identity {
  match elem {
    CurveType::Line(line) => line,
    CurveType::Arc(arc) => arc,
    CurveType::Circle(circle) => circle,
    CurveType::BezierSpline(spline) => spline,
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use shapex::test_data;

  fn make_sketch(lines: &Vec<Line>) -> Sketch {
    let mut sketch = Sketch::default();
    for line in lines {
      sketch.elements.push(rc(line.clone().into_enum()));
    }
    sketch
  }

  fn split_all(sketch: &Sketch) -> Vec<CurveType> {
    Sketch::all_split(&sketch.elements).into_iter()
    .map(|elem| elem.cache )
    .collect()
  }

  #[test]
  fn split_all_crossing() {
    let sketch = make_sketch(&test_data::crossing_lines());
    let segments = split_all(&sketch);
    assert_eq!(segments.len(), 4, "{} segments found instead of 4", segments.len());
    assert_eq!(segments[0].as_curve().length(), 0.5, "Segment had wrong length");
    assert_eq!(segments[1].as_curve().length(), 0.5, "Segment had wrong length");
    assert_eq!(segments[2].as_curve().length(), 0.5, "Segment had wrong length");
    assert_eq!(segments[3].as_curve().length(), 0.5, "Segment had wrong length");
  }

  #[test]
  fn split_all_parallel() {
    let sketch = make_sketch(&test_data::parallel_lines());
    let segments = split_all(&sketch);
    assert_eq!(segments.len(), 2, "{} segments found instead of 2", segments.len());
    assert_eq!(segments[0].as_curve().length(), 1.0, "Segment had wrong length");
    assert_eq!(segments[1].as_curve().length(), 1.0, "Segment had wrong length");
  }

  #[test]
  fn t_split() {
    let sketch = make_sketch(&test_data::t_section());
    let segments = split_all(&sketch);
    assert_eq!(segments.len(), 3, "{} segments found instead of 3", segments.len());
    assert_eq!(segments[0].as_curve().length(), 1.0, "Segment had wrong length");
    assert_eq!(segments[1].as_curve().length(), 1.0, "Segment had wrong length");
    assert_eq!(segments[2].as_curve().length(), 1.0, "Segment had wrong length");
  }

  #[test]
  fn region_rect() {
    let sketch = make_sketch(&test_data::rectangle());
    let cut_elements = Sketch::all_split(&sketch.elements);
    assert_eq!(cut_elements.len(), 4, "{} cut_elements found instead of 4", cut_elements.len());
    let islands = Sketch::build_islands(&cut_elements);
    let regions = Sketch::get_regions(cut_elements, false);
    assert_eq!(islands.len(), 1, "{} islands found instead of 1", islands.len());
    assert_eq!(regions.len(), 1, "{} regions found instead of 1", regions.len());
  }

  #[test]
  fn region_crossing_rect() {
    let sketch = make_sketch(&test_data::crossing_rectangle());
    let cut_elements = Sketch::all_split(&sketch.elements);
    assert_eq!(cut_elements.len(), 8, "{} cut_elements found instead of 8", cut_elements.len());
    let islands = Sketch::build_islands(&cut_elements);
    let regions = Sketch::get_regions(cut_elements, false);
    assert_eq!(islands.len(), 1, "{} islands found instead of 1", islands.len());
    assert_eq!(regions.len(), 1, "{} regions found instead of 1", regions.len());
  }

  #[test]
  fn region_crossing_corner() {
    let mut lines = test_data::rectangle();
    lines[2].points.1.x = -2.0;
    lines[3].points.0.y = -2.0;
    let sketch = make_sketch(&lines);
    let cut_elements = Sketch::all_split(&sketch.elements);
    assert_eq!(cut_elements.len(), 6, "{} cut_elements found instead of 6", cut_elements.len());
    let islands = Sketch::build_islands(&cut_elements);
    let regions = Sketch::get_regions(cut_elements, false);
    assert_eq!(islands.len(), 1, "{} islands found instead of 1", islands.len());
    assert_eq!(regions.len(), 1, "{} regions found instead of 1", regions.len());
  }

  #[test]
  fn region_arc_rect() {
    let data = test_data::arc_rectangle();
    let mut sketch = Sketch::default();
    for curve in data {
      sketch.elements.push(rc(curve));
    }
    let cut_elements = Sketch::all_split(&sketch.elements);
    assert_eq!(sketch.elements.len(), 4, "{} elements found instead of 4", sketch.elements.len());
    assert_eq!(cut_elements.len(), 4, "{} cut_elements found instead of 4", cut_elements.len());
    let islands = Sketch::build_islands(&cut_elements);
    assert_eq!(islands.len(), 1, "{} islands found instead of 1", islands.len());
    let regions = Sketch::get_regions(cut_elements, false);
    assert_eq!(regions.len(), 1, "{} regions found instead of 1", regions.len());
  }

  #[test]
  fn region_rect_split_diagonal() {
    let mut sketch = make_sketch(&test_data::rectangle());
    let line = Line::new(Point3::new(-1.0, 1.0, 0.0), Point3::new(1.0, -1.0, 0.0));
    sketch.elements.push(rc(line.into_enum()));
    let cut_elements = Sketch::all_split(&sketch.elements);
    let regions = Sketch::get_regions(cut_elements, false);
    assert_eq!(regions.len(), 2, "{} regions found instead of 2", regions.len());
  }

  #[test]
  fn dangling_segment() {
    let mut sketch = Sketch::default();
    let line = Line::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));
    sketch.elements.push(rc(line.into_enum()));
    let cut_elements = Sketch::all_split(&sketch.elements);
    let _regions = Sketch::get_regions(cut_elements, false);
  }

  #[test]
  fn circle_in_circle_profile() {
    let mut sketch = Sketch::default();
    let circle = Circle::new(Point3::new(-27.0, 3.0, 0.0), 68.97340462273907);
    let inner_circle = Circle::new(Point3::new(-1.0, 27.654544570311774, 0.0), 15.53598031475424);
    sketch.elements.push(rc(circle.into_enum()));
    sketch.elements.push(rc(inner_circle.into_enum()));
    let regions = sketch.get_profiles(false);
    assert_eq!(regions.len(), 2);
    assert_eq!(regions[0].len(), 2);
    assert_eq!(regions[1].len(), 1);
  }
}
