use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;

use solvo::*;
use shapex::*;

use crate::utils::vec_to_js;
use crate::utils::point_to_js;
use crate::buffer_geometry::JsBufferGeometry;


#[wasm_bindgen]
pub struct JsRegion {
  #[wasm_bindgen(skip)]
  pub profile: Profile,

  #[wasm_bindgen(skip)]
  pub plane: Matrix4,

  #[wasm_bindgen(skip)]
  pub component: Rc<RefCell<Component>>,
}

#[wasm_bindgen]
impl JsRegion {
  pub fn get_mesh(&self) -> JsBufferGeometry {
    web_sys::console::time_with_label("tesselate_profile");
    let transform = self.plane.invert().unwrap();
    let mut profile = self.profile.clone();
    for wire in &mut profile {
      for curve in wire {
        curve.transform(&transform);
      }
    }
    let mut mesh = geom2d::tesselate_profile(&profile, Vec3::unit_z());
    mesh.transform(&self.plane);
    let geom = JsBufferGeometry::from(mesh.to_buffer_geometry());
    web_sys::console::time_end_with_label("tesselate_profile");
    geom
  }

  pub fn get_center(&self) -> JsValue {
    let center = self.profile[0].iter().fold(
      Point3::origin(),
      |acc, elem| acc + match &elem.base {
        CurveType::Circle(circle) => circle.center.to_vec() * 2.0,
        _ => elem.bounds.0.to_vec() + elem.bounds.1.to_vec(),
      }
    ) / (self.profile[0].len() as f64 * 2.0);
    point_to_js(center)
  }

    pub fn get_normal(&self) -> JsValue {
    let normal = self.plane.transform_vector(Vec3::new(0.0, 0.0, 1.0));
    vec_to_js(normal)
  }

  pub fn extrude(&self, distance: f64) {
    web_sys::console::time_with_label("BREP extrude");
    let tool = features::extrude(&self.profile, distance).unwrap();
    self.component.borrow_mut().compound.add(tool.into_compound());
    web_sys::console::time_end_with_label("BREP extrude");
  }

  pub fn extrude_preview(&self, distance: f64) -> JsValue {
    let extrusion = features::extrude(&self.profile, distance);
    match extrusion {
      Ok(res) => JsValue::from(JsBufferGeometry::from_solid(&res)),
      Err(error) => JsValue::from(error),
    }
  }
}