import * as THREE from 'three'

const snapDistance = 14 // px
const maxSnapReferences = 5

export class Snapper {
  constructor(viewport, updateView) {
    this.viewport = viewport
    this.updateView = updateView
    this.guides = []
    this.lastSnaps = []
    this.snapAnchor = null
  }

  reset() {
    this.guides = []
    this.snapAnchor = null
    this.lastSnaps = []
    this.updateView([], null)
  }

  snap(vec, coords) {
    if(this.viewport.activeTool.enableSnapping) {
      this.guides = []
      vec = this.snapToGuides(vec) || vec
      this.catchSnapPoints(coords)
      this.updateView(this.guides, this.snapAnchor)
    }
    return [vec, coords]
  }

  getSnapPoints() {
    const sketchElements = this.viewport.activeComponent.get_sketch().get_sketch_elements()
    return sketchElements.flatMap(elem => {
      let points = elem.get_snap_points().map(p => new THREE.Vector3().fromArray(p))
      // Filter out last point of the sketch element actively being drawn
      if(elem == sketchElements.slice(-1)[0]) {
        const handles = elem.get_handles()
        const lastHandle = new THREE.Vector3().fromArray(handles[handles.length - 1])
        points = points.filter(p => !p.equals(lastHandle))
      }
      // Filter out handle actively being dragged
      if(this.viewport.activeHandle && elem.id() == this.viewport.activeHandle.elem.id()) {
        const handlePoint = new THREE.Vector3().fromArray(this.viewport.activeHandle.elem.get_handles()[this.viewport.activeHandle.index])
        points = points.filter(p => !p.equals(handlePoint))
      }
      return points
    })
  }

  catchSnapPoints(coords) {
    const snapPoints = this.getSnapPoints()
    let closestDist = 99999
    let target
    snapPoints.forEach(p => {
      const dist = this.viewport.renderer.toScreen(p).distanceTo(coords)
      if(dist < snapDistance && dist < closestDist) {
        closestDist = dist
        target = p
      }
    })
    if(!target) return
    if(!(this.lastSnaps[0] && this.lastSnaps[0].equals(target))) {
      this.lastSnaps.unshift(target)
      if(this.lastSnaps.length >= maxSnapReferences) this.lastSnaps.pop()
    }
  }

  snapToGuides(vec) {
    if(!vec) return
    const screenVec = this.viewport.renderer.toScreen(vec)
    let snapX
    this.lastSnaps.some(snap => {
      // Compare world space X axis..
      const testSnap = snap.clone()
      testSnap.setY(vec.y)
      testSnap.setZ(vec.z)
      const screenSnap = this.viewport.renderer.toScreen(testSnap)
      // .. in screen space
      if(screenVec.distanceTo(screenSnap) < snapDistance) {
        snapX = snap
        return true
      }
    })
    let snapY
    this.lastSnaps.some(snap => {
      const testSnap = snap.clone()
      testSnap.setX(vec.x)
      testSnap.setZ(vec.z)
      const screenSnap = this.viewport.renderer.toScreen(testSnap)
      if(screenVec.distanceTo(screenSnap) < snapDistance) {
        snapY = snap
        return true
      }
    })
    const snapVec = new THREE.Vector3(snapX ? snapX.x : vec.x, snapY ? snapY.y : vec.y, vec.z)
    const screenSnapVec = this.viewport.renderer.toScreen(snapVec)
    if(snapX) {
      const start = this.viewport.renderer.toScreen(snapX)
      this.guides.push({
        id: 'v' + start.x + start.y,
        start,
        end: screenSnapVec,
      })
    }
    if(snapY) {
      const start = this.viewport.renderer.toScreen(snapY)
      this.guides.push({
        id: 'h' + start.x + start.y,
        start,
        end: screenSnapVec,
      })
    }
    if(snapX && snapY) {
      if(this.snapAnchor && this.snapAnchor.vec.equals(snapVec)) return snapVec
      this.snapAnchor = {
        type: 'snap',
        pos: this.viewport.renderer.toScreen(snapVec),
        vec: snapVec,
        id: '' + snapVec.x + snapVec.y + snapVec.z,
      }
      return snapVec
    } else {
      this.snapAnchor = null
    }
    if(snapX || snapY) return snapVec
  }
}
