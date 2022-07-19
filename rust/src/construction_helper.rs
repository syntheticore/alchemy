use wasm_bindgen::prelude::*;

use solvo::*;
use shapex::*;
use shapex::internal::Ref;

use crate::utils::matrix_to_js;
use crate::feature::JsPlanarRef;
use crate::feature::JsAxialRef;

// use crate::log;


#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct JsConstructionHelper {

  #[wasm_bindgen(skip)]
  pub real: Ref<ConstructionHelper>,
}

impl JsConstructionHelper {
  pub fn new(helper: &Ref<ConstructionHelper>) -> Self {
    JsConstructionHelper {
      real: helper.clone(),
    }
  }
}

#[wasm_bindgen]
impl JsConstructionHelper {
  pub fn get_transform(&self) -> JsValue {
    let m = match &self.real.borrow().helper_type {
      ConstructionHelperType::Plane(plane) => plane.as_transform(),
      _ => Matrix4::one(),
    };
    matrix_to_js(m)
  }

  pub fn make_planar_reference(&self) -> JsValue {
    let helper = self.real.borrow();
    match &helper.helper_type {
      ConstructionHelperType::Plane(_) => JsValue::from(JsPlanarRef::new(PlanarRef::HelperRef(self.real.clone()))),
      _ => unreachable!(),
    }
  }

    pub fn make_axial_reference(&self) -> JsValue {
    let helper = self.real.borrow();
    match &helper.helper_type {
      ConstructionHelperType::Axis(_) => JsValue::from(JsAxialRef::new(AxialRef::HelperRef(self.real.clone()))),
      _ => unreachable!(),
    }
  }

  pub fn duplicate(&self) -> JsConstructionHelper {
    self.clone()
  }
}
