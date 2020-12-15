import * as THREE from 'three'

class Tool {
  constructor(component, viewport) {
    this.component = component
    this.viewport = viewport
  }

  click() {}

  mouseDown() {}

  mouseMove() {}

  dispose() {}
}


class HighlightTool extends Tool {
  constructor(component, viewport, selectors) {
    super(component, viewport)
    this.selectors = selectors
  }

  mouseMove(vec, coords) {
    const object = this.viewport.objectsAtScreen(coords, this.selectors)[0]
    if(!object) return this.viewport.render()
    const oldMaterial = object.material
    object.material = {
      curve: this.viewport.highlightLineMaterial,
      region: this.viewport.highlightRegionMaterial,
      face: this.viewport.highlightSurfaceMaterial,
    }[object.alcType]
    this.viewport.render()
    object.material = oldMaterial
  }
}


export class ManipulationTool extends HighlightTool {
  constructor(component, viewport) {
    super(component, viewport, ['curve'])
  }

  click(coords) {
    const object = this.viewport.objectsAtScreen(coords, this.selectors)[0]
    if(object) return this.viewport.render()
    if(this.viewport.selectedElement) {
      const mesh = this.viewport.document.data[this.viewport.selectedElement.id()]
      mesh.material = this.viewport.lineMaterial
    }
    this.viewport.$emit('element-selected', null)
    this.viewport.transformControl.detach()
    this.viewport.render()
  }

  mouseDown(vec, coords) {
    const object = this.viewport.objectsAtScreen(coords, this.selectors)[0]
    if(!object) return
    if(this.viewport.selectedElement) {
      const mesh = this.viewport.document.data[this.viewport.selectedElement.id()]
      mesh.material = this.viewport.lineMaterial
    }
    object.material = this.viewport.selectionLineMaterial
    this.viewport.$emit('element-selected', object.alcElement)
    // this.viewport.transformControl.attach(object)
    this.viewport.render()
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


export class TrimTool extends Tool {
  click(coords) {
    const object = this.viewport.objectsAtScreen(coords, 'curve')[0]
    if(object) return this.viewport.render()

    this.viewport.render()
  }

  mouseDown(vec, coords) {
    const object = this.viewport.objectsAtScreen(coords, 'curve')[0]
    if(!object) return this.viewport.render()

    this.viewport.render()
  }

  mouseMove(vec, coords) {
    const object = this.viewport.objectsAtScreen(coords, 'curve')[0]
    if(!object) return this.viewport.render()

    this.viewport.render()
  }
}


class SelectionTool extends HighlightTool {
  constructor(component, viewport, selectors, callback) {
    super(component, viewport, selectors)
    this.callback = callback
  }

  mouseDown(vec, coords) {
    const mesh = this.viewport.objectsAtScreen(coords, this.selectors)[0]
    if(!mesh) return
    const selection = this.select(mesh)
    this.callback(selection, mesh)
  }
}


export class ObjectSelectionTool extends SelectionTool {
  constructor(component, viewport, callback) {
    super(component, viewport, ['curve'], callback)
  }

  select(mesh) {
    return mesh.alcElement
  }
}


export class ProfileSelectionTool extends SelectionTool {
  constructor(component, viewport, callback) {
    super(component, viewport, ['region'], callback)
  }

  select(mesh) {
    return mesh.alcRegion
  }
}


export class FaceSelectionTool extends SelectionTool {
  constructor(component, viewport, callback) {
    super(component, viewport, ['face'], callback)
  }

  select(mesh) {
    return mesh.alcFaceId
  }
}


export class LineTool extends Tool {
  mouseDown(vec) {
    this.mouseMove(vec)
    this.line = this.component.get_sketch().add_line(vec.toArray(), vec.toArray())
    this.viewport.elementChanged(this.line, this.component)
    // Restart tool when we close a loop
    if(this.firstPoint && vec.equals(this.firstPoint)) {
      this.component.get_sketch().remove_element(this.line.id())
      this.line = null
      this.firstPoint = null
      return
    }
    if(!this.firstPoint) this.firstPoint = vec
  }

  mouseMove(vec) {
    if(!this.line) return
    let p1 = this.line.get_handles()[0]
    this.line.set_handles([p1, vec.toArray()])
    this.viewport.elementChanged(this.line, this.component)
  }

  dispose() {
    if(!this.line) return
    this.component.get_sketch().remove_element(this.line.id())
    this.viewport.componentChanged(this.component)
  }
}


export class SplineTool extends Tool {
  mouseDown(vec) {
    if(this.spline) {
      let points = this.spline.get_handles()
      points[points.length - 1] = vec.toArray()
      points.push(vec.toArray())
      this.spline.set_handles(points)
    } else {
      this.spline = this.component.get_sketch().add_spline([vec.toArray(), vec.toArray()])
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


export class CircleTool extends Tool {
  mouseDown(vec) {
    if(this.center) {
      this.center = null
      this.circle = null
    } else {
      this.center = vec
      this.circle = this.component.get_sketch().add_circle(vec.toArray(), 1)
    }
  }

  mouseMove(vec) {
    if(!this.center) return
    this.circle.set_handles([this.center.toArray(), vec.toArray()])
    this.viewport.elementChanged(this.circle, this.component)
  }
}


export class ArcTool extends Tool {
  mouseDown(vec) {
    if(this.start && this.end && this.middle) {
      this.start = null
      this.end = null
      this.middle = null
    } else if(this.start && this.end) {
      this.middle = vec
    } else if(this.start) {
      this.end = vec
    } else {
      this.start = vec
      this.arc = this.component.get_sketch().add_arc(vec.toArray(), vec.toArray(), vec.toArray())
    }
  }

  mouseMove(vec) {
    if(!this.start) return
    this.arc.set_handles([this.center.toArray(), vec.toArray()])
    this.viewport.elementChanged(this.arc, this.component)
  }
}
