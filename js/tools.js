import * as THREE from 'three'

import {
  vec2three,
  matrix2three,
  matrixFromThree,
  rotationFromNormal
} from './utils.js'

class Tool {
  constructor(component, viewport) {
    this.component = component
    this.viewport = viewport
    this.enableSnapping = false
  }

  click() {}

  mouseDown() {}

  mouseUp() {}

  mouseMove() {}

  dispose() {}
}


class HighlightTool extends Tool {
  constructor(component, viewport, selectors) {
    super(component, viewport)
    this.selectors = selectors
  }

  mouseMove(vec, coords) {
    if(this.viewport.hoveredHandle) return this.viewport.renderer.render()
    const object = this.viewport.renderer.objectsAtScreen(coords, this.selectors)[0]
    this.viewport.renderer.render()
    this.viewport.$emit('update:highlight', object && object.alcObject)
  }

  getObject(coords, any) {
    return new Promise(resolve => {
      const items = this.viewport.renderer.objectsAtScreen(coords, this.selectors)
        .map(obj => obj.alcObject)
      if(items.length > 1 && !any) {
        // Combat close-widgets event
        setTimeout(() => this.viewport.widgets.push({
          items,
          pos: coords,
          cb: (choice) => resolve(choice),
        }))
      } else {
        resolve(items[0])
      }
    })
  }
}


export class ManipulationTool extends HighlightTool {
  constructor(component, viewport) {
    super(component, viewport, ['curve'])
  }

  click(vec, coords) {
    this.getObject(coords, true).then(curve => {
      if(curve || this.viewport.$root.isCtrlPressed) return this.viewport.renderer.render()
      this.viewport.$emit('update:selection', this.viewport.selection.clear())
      // this.viewport.renderer.removeGizmo()
    })
  }

  mouseDown(vec, coords) {
    if(this.viewport.activeHandle) {
      this.enableSnapping = true
      return
    }
    this.getObject(coords).then(curve => {
      if(!curve) return
      this.viewport.$emit('update:selection', this.viewport.selection.handle(curve, this.viewport.$root.isCtrlPressed))
      // this.viewport.gizmo.attach(object)
    })
  }

  mouseUp(vec, coords) {
    this.enableSnapping = false
  }

  mouseMove(vec, coords) {
    const handle = this.viewport.activeHandle
    if(handle) {
      let handles = handle.elem.get_handles()
      handles[handle.index] = vec.toArray()
      handle.elem.set_handles(handles)
      this.viewport.elementChanged(handle.elem, this.component)
    } else {
      super.mouseMove(vec, coords)
    }
  }
}


// export class PlaneTool extends HighlightTool {
//   constructor(component, viewport) {
//     super(component, viewport, ['face'])
//   }

//   click(vec, coords) {
//     const face = this.viewport.renderer.objectsAtScreen(coords, this.selectors)[0]
//     if(face && face.alcObject.get_surface_type() == 'Planar') {
//       const position = vec2three(face.alcObject.get_origin())
//       let rotation = rotationFromNormal(vec2three(face.alcObject.get_normal()))

//       this.viewport.renderer.sketchPlane.position = position
//       this.viewport.renderer.sketchPlane.rotation.setFromRotationMatrix(rotation)

//       rotation.setPosition(position)
//       this.viewport.snapper.planeTransform = rotation
//       this.component.real.get_sketch().set_workplane(matrixFromThree(rotation))

//       this.viewport.regionsDirty = true
//       this.viewport.updateRegions()
//     }
//     this.viewport.renderer.render()
//   }

//   mouseDown(vec, coords) {
//     const face = this.viewport.renderer.objectsAtScreen(coords, this.selectors)[0]
//     this.viewport.renderer.render()
//   }
// }


export class TrimTool extends Tool {
  click(vec, coords) {
    const curve = this.viewport.renderer.objectsAtScreen(coords, 'curve')[0]
    if(curve) return this.viewport.renderer.render()

    this.viewport.renderer.render()
  }

  mouseDown(vec, coords) {
    const curve = this.viewport.renderer.objectsAtScreen(coords, 'curve')[0]
    if(!curve) return this.viewport.renderer.render()

    this.viewport.renderer.render()
  }

  mouseMove(vec, coords) {
    const curve = this.viewport.renderer.objectsAtScreen(coords, 'curve')[0]
    if(!curve) return this.viewport.renderer.render()

    this.viewport.renderer.render()
  }
}


class PickTool extends HighlightTool {
  constructor(component, viewport, selectors, callback) {
    super(component, viewport, selectors)
    this.callback = callback
  }

  mouseDown(vec, coords) {
    const mesh = this.viewport.renderer.objectsAtScreen(coords, this.selectors)[0]
    if(!mesh) return
    const selection = this.select(mesh)
    this.callback(selection, mesh)
  }

  select(mesh) {
    return mesh.alcObject
  }
}


export class ObjectPickTool extends PickTool {
  constructor(component, viewport, callback) {
    super(component, viewport, ['curve'], callback)
  }
}


export class ProfilePickTool extends PickTool {
  constructor(component, viewport, callback) {
    super(component, viewport, ['region'], callback)
  }
}


export class FacePickTool extends PickTool {
  constructor(component, viewport, callback) {
    super(component, viewport, ['face'], callback)
  }
}

export class PlanePickTool extends PickTool {
  constructor(component, viewport, callback) {
    super(component, viewport, ['plane', 'face'], callback)
  }
}


export class SketchTool extends Tool {
  constructor(component, viewport, sketch) {
    super(component, viewport)
    this.sketch = sketch
    this.enableSnapping = true
  }
}


export class LineTool extends SketchTool {
  constructor(component, viewport, sketch) {
    super(component, viewport, sketch)
  }

  mouseDown(vec) {
    this.mouseMove(vec)
    const elems = this.sketch.get_sketch_elements()
    elems.pop()
    const touchesExisting = elems
      .flatMap(elem => elem.get_snap_points() )
      .map(p => vec2three(p) )
      .some(p => p.equals(vec) )
    // Restart tool when we hit an existing point
    if(touchesExisting && this.line) {
      this.line = null
    } else {
      this.line = this.sketch.add_line(vec.toArray(), vec.toArray())
      this.viewport.elementChanged(this.line, this.component)
    }
  }

  mouseMove(vec) {
    if(!this.line) return
    let p1 = this.line.get_handles()[0]
    this.line.set_handles([p1, vec.toArray()])
    this.viewport.elementChanged(this.line, this.component)
  }

  dispose() {
    if(!this.line) return
    this.line.remove()
    this.viewport.componentChanged(this.component)
  }
}


export class SplineTool extends SketchTool {
  constructor(component, viewport, sketch) {
    super(component, viewport, sketch)
  }

  mouseDown(vec) {
    if(this.spline) {
      let points = this.spline.get_handles()
      points[points.length - 1] = vec.toArray()
      points.push(vec.toArray())
      this.spline.set_handles(points)
    } else {
      this.spline = this.sketch.add_spline([vec.toArray(), vec.toArray()])
    }
    this.viewport.elementChanged(this.spline, this.component)
  }

  mouseMove(vec) {
    if(!this.spline) return
    let points = this.spline.get_handles()
    points[points.length - 1] = vec.toArray()
    this.spline.set_handles(points)
    this.viewport.elementChanged(this.spline, this.component)
  }

  dispose() {
    if(!this.spline) return
    let points = this.spline.get_handles()
    points.pop()
    this.spline.set_handles(points)
    this.viewport.elementChanged(this.spline, this.component)
  }
}


export class CircleTool extends SketchTool {
  constructor(component, viewport, sketch) {
    super(component, viewport, sketch)
  }

  mouseDown(vec) {
    if(this.center) {
      this.center = null
      this.circle = null
    } else {
      this.center = vec
      this.circle = this.sketch.add_circle(vec.toArray(), 1)
    }
  }

  mouseMove(vec) {
    if(!this.center) return
    this.circle.set_handles([this.center.toArray(), vec.toArray()])
    this.viewport.elementChanged(this.circle, this.component)
  }
}


export class ArcTool extends SketchTool {
  constructor(component, viewport, sketch) {
    super(component, viewport, sketch)
  }

  mouseDown(vec) {
    if(this.start && this.end) {
      this.start = null
      this.end = null
      this.arc = null
    } else if(this.start) {
      this.end = vec
    } else {
      this.start = vec
    }
  }

  mouseMove(vec) {
    if(!this.start || !this.end) return
    this.arc = this.arc || this.sketch.add_arc(
      this.start.toArray(),
      vec.toArray(),
      this.end.toArray()
    )
    this.arc.set_initial_handles([this.start.toArray(), vec.toArray(), this.end.toArray()])
    this.viewport.elementChanged(this.arc, this.component)
  }
}
