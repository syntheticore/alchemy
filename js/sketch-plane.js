import * as THREE from 'three'

import { matrix2three } from './utils.js'

export default class SketchPlane extends THREE.Object3D {
  constructor(camera) {
    super()

    // Grid
    this.cachedVec = new THREE.Vector3()
    this.update(camera)

    // Axis Helper
    this.add(new THREE.AxesHelper(10.0))

    // Click Catcher
    var groundGeo = new THREE.PlaneBufferGeometry(99999, 99999)
    // groundGeo.rotateX(- Math.PI / 2)
    var ground = new THREE.Mesh(groundGeo, new THREE.MeshBasicMaterial({
      side: THREE.DoubleSide,
      transparent: true,
      opacity: 0.0,
      depthWrite: false,
    }))
    ground.alcProjectable = true
    this.add(ground)

    this.visible = false
  }

  setPlane(plane) {
    this.position.setFromMatrixPosition(plane)
    this.rotation.setFromRotationMatrix(plane)
  }

  update(camera) {
    const pos = this.grid && this.grid.position || new THREE.Vector3()
    const dist = (pos).distanceTo(camera.position) //XXX should be distance to plane
    const size = Math.pow(10, String(Math.round(10 * dist / 4)).length) / 10
    if(size != this.lastSize) {
      this.remove(this.grid)
      const multiple = 3
      this.grid = new THREE.GridHelper(size * multiple, 10 * multiple)
      this.grid.rotateX(Math.PI / 2) //INFO move to XY plane for cross product with Z-Axis to work
      this.grid.material.opacity = 0.1
      this.grid.material.transparent = true
      this.add(this.grid)
      this.lastSize = size
    }
    // Move grid towards camera to avoid z-fighting
    const lookVec = camera.getWorldDirection(this.cachedVec)
    const facing = this.grid.up.dot(lookVec)
    //XXX transform https://stackoverflow.com/questions/35641875/three-js-how-to-find-world-orientation-vector-of-objects-local-up-vector
    this.grid.position.z = -0.004 * dist * facing
  }
}
