export default class Component {
  constructor(realComponent, parent, componentData) {
    this.real = realComponent
    this.parent = parent

    this.id = realComponent.id()

    this.UIData = componentData[this.id] || {
      title: parent ? "New Component" : "Main Assembly",
      hidden: false,
      material: null,
      cog: false,
      sectionViews: [],
      parameters: [],
      exportConfigs: [],
    }
    componentData[this.id] = this.UIData

    this.solids = []
    this.sketches = []
    this.helpers = []
    this.children = []
    this.update(componentData)

    const cache = {
      faces: [],
      edges: [],
      regions: [],
      curves: [],
    }
    // Hide cache from Vue
    this.cache = () => cache
  }

  typename() {
    return 'Component'
  }

  update(componentData) {
    this.updateChildren(componentData)
    // this.updateSolids()
    this.updateSketches()
    this.updateHelpers()
  }

  updateChildren(componentData) {
    this.freeChildren()
    this.children = this.real.get_children().map( realChild => new Component(realChild, this, componentData) )
  }

  updateSolids() {
    this.freeSolids()
    this.solids = this.real.get_solids()
    this.solids.forEach(solid => solid.component = this )
  }

  updateSketches() {
    this.freeSketches()
    this.sketches = this.real.get_sketches()
  }

  updateHelpers() {
    this.freeHelpers()
    this.helpers = this.real.get_planes()
    this.helpers.forEach(helper => helper.component = this )
  }

  freeChildren() {
    this.children.forEach(child => child.free() )
    this.children = []
  }

  freeSolids() {
    this.solids.forEach(solid => {
      solid.free()
      solid.deallocated = true
    })
    this.solids = []
  }

  freeSketches() {
    this.sketches.forEach(sketch => sketch.free() )
    this.sketches = []
  }

  freeHelpers() {
    this.helpers.forEach(helper => helper.free() )
    this.helpers = []
  }

  free(keepSelf) {
    this.freeChildren()
    this.freeSolids()
    this.freeSketches()
    this.freeHelpers()
    if(!this.real || keepSelf) return
    this.real.free()
    this.real = null
  }

  findChild(id) {
    if(this.id == id) return this
    for(let child of this.children) {
      const found = child.findChild(id)
      if(found) return found
    }
  }

  findSketch(id) {
    const sketch = this.sketches.find(sketch => sketch.id() == id )
    if(sketch) return sketch
    for(let child of this.children) {
      const found = child.findSketch(id)
      if(found) return found
    }
  }

  findSketchByFeature(id) {
    const sketch = this.sketches.find(sketch => sketch.get_feature_id() == id )
    if(sketch) return sketch
    for(let child of this.children) {
      const found = child.findSketchByFeature(id)
      if(found) return found
    }
  }

  getMaterial() {
    return this.material || (this.parent && this.parent.getMaterial())
  }

  // Returns zero for empty components,
  // but undefined when weight could not be determined
  getWeight() {
    if(this.solids.length && !this.material) return
    try {
      let weight = this.children.reduce((acc, child) => {
        const childWeight = child.getWeight()
        if(childWeight === undefined) throw 'no weight'
        return acc + childWeight
      }, 0.0)
      return weight + (this.material ? this.getVolume() * this.material.density : 0.0)
    } catch(e) {
      if(e !== 'no weight') throw e
    }
  }

  getVolume() {
    return this.solids.reduce((acc, solid) => acc + solid.volume, 0.0)
  }

  hasAncestor(parent) {
    if(this === parent) return true
    return this.parent && this.parent.hasAncestor(parent)
  }

  getParameters() {
    const params = [...this.UIData.parameters]
    const parentParams = this.parent ? this.parent.getParameters() : []
    parentParams.forEach(other => {
      const index = params.findIndex(own => own.name == other.name)
      if(index == -1) params.push(other)
    })
    return params
  }

  // isHidden() {
  //   return this.hidden || (this.parent && this.parent.isHidden())
  // }

  // serialize() {
  //   return {
  //     title: this.title,
  //     hidden: this.hidden,
  //     cog: this.cog,
  //     sectionViews: this.sectionViews,
  //     parameters: this.UIData.parameters,
  //     exportConfigs: this.exportConfigs,
  //     real: this.real.serialize(),
  //     children: this.children.map(child => child.serialize() ),
  //   }
  // }

  // unserialize(dump) {
  //   console.log(dump)
  //   this.title = dump.title
  //   this.hidden = dump.hidden
  //   this.cog = dump.cog
  //   this.sectionViews = dump.sectionViews || []
  //   this.UIData.parameters = dump.parameters || []
  //   this.exportConfigs = dump.exportConfigs || []
  //   this.real.unserialize(dump.real)
  //   dump.children.forEach(childDump => {
  //     let child = this.createComponent()
  //     child.unserialize(childDump)
  //   })
  // }
}
