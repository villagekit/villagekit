# VillageKit

Village Kit is an open source code-as-CAD system for makers.

There are 5 types of products (so far):

- Solid: A 3d object printed or CNC'd
- Sheet: A 2d object cut from sheet material
- Stock: An off-the-shelf product
- Assembly: A composition of other products
- Library: A re-usable trait or object of functions

Products are written as [Rimu code](https://rimu.dev).

Products can be shared and imported, similar to npm.

## Notes

???

Data types:

- `ProductObject`
  - `design`
  - `params`
- `ProductDesign`
  - `::Solid`
    - `solid(params => 3d)`
  - `::Sheet`
    - `sheet(params => 2d)`
  - `::Stock`
    - `stock(params => meshes, materials, instances)`
  - `::Assembly`
    - `assembly(params => [products])`
- `ProductTransform`
  - `translate`
  - `rotate`
  - `scale`
- `ProductDesignCode`
  - `path`
  - `code`
- `ProductDesignRender`
  - `meshes`
  - `materials`
  - `instances`

  - `[PartType]`: e.g. "grid beam"
    - `[Part]`: `Transform`
      - `PartSpec`: e.g. "a grid beam of length 10"
    - `[PartRender]`
      - `[PartMeshParams]`
        - `PartMeshHandle`
      - `[PartMaterialParams]`
        - `PartMaterialHandle`
      - `[PartInstance]`: Transform

Data Flow:

- Load workspace
- List products in workspace
- Load product metadata (`villagekit.toml`)
- Load product
  - Load and parse entry code file
    - Scan for imports, recursively load and parse imports
      - If part, load part
    - Resolve into one AST document.
  - Send parameters to product, receive result

User code class types:

- Parametric
  - `parameters`
  - `presets`
- Transformable
  - `transform`
- ProductBase: Parametric, Transformable
- Solid: ProductBase
  - `solid(params => 3d)`
- Sheet: ProductBase
  - `sheet(params => 2d)`
- Stock: ProductBase
  - `stock(params => ...`
    - `3d`: how to render in 3d
      - `meshes`
      - `materials`
      - `instances`
    - TODO: `2d`: how to render in 2d
    - TODO: `cut`: how to cut larger sizes to smaller sizes
- Assembly: ProductBase
  - `assembly(params => [products])`


Example assembly product: `chair.rimu`

TODO: question: Should assembly products be a class too?

```
import Assembly from "@stdlib/assembly"
import GridBeam from "@villagekit/gridbeam"

export Assembly
  parameters:
    seat_width:
      label: 'Seat width'
      type: 'number'
      min: 5
      max: 10
      step: 5
    seat_depth:
      label: 'Seat depth'
      type: 'number'
      min: 5
      max: 15
    seat_height:
      label: 'Seat height'
      description: 'The height from the ground to the top of the seat'
      type: 'number'
      min: 5
      max: 15
    should_include_back:
      label: 'Include back'
      type: 'boolean'
    backHeight:
      label: 'Back height',
      description: 'The height from the seat to the top of the backrest'
      type: 'number'
      min: 5
      max: 10

  presets:
    - id: 'regular-with-back'
      label: 'Regular With Back'
      values:
        backHeight: 10
        seatDepth: 10
        seatHeight: 10
        seatWidth: 10
        shouldIncludeBack: true
    - id: 'regular'
      label: 'Regular (Without Back)'
      values:
        backHeight: 10
        seatDepth: 10
        seatHeight: 10
        seatWidth: 10
        shouldIncludeBack: false

  plugins: ['smart-fasteners']

  parts: (parameters) =>
    let
      seat_width:
      seat_depth:
      seat_height:
      back_height:
      should_include_back:
        = parameters

      back_z_beam_end_z = if should_include_back then seat_height + back_height else seat_height
      seat_panel_start_y = if should_include_back then -1 else 0
      seat_panel_end_y = if should_include_back then seat_depth - 1 else seat_depth
    in
      - GridPanel.XY
          x: [0, seatWidth]
          y: [seatPanelStartY, seatPanelEndY]
          z: seatHeight

      - and
          shouldIncludeBack
          GridPanel.XZ
            x: [0, seatWidth]
            y: seatDepth - 2
            z: [seatHeight + 1, seatHeight + 1 + backHeight]
            fit: 'top'

      - GridBeam.Z
          x: 0
          y: 0
          z: [0, seatHeight]

      - GridBeam.Z
          x: seatWidth - 1
          y: 0
          z: [0, seatHeight]

      - GridBeam.Z
          x: 0
          y: seatDepth - 1
          z: [0, backZBeamEndZ]

      - GridBeam.Z
          x: seatWidth - 1
          y: seatDepth - 1
          z: [0, backZBeamEndZ]

      - GridBeam.X
          x: [0, seatWidth]
          y: 1
          z: seatHeight - 2

      - GridBeam.X
          x: [0, seatWidth]
          y: seatDepth - 2
          z: seatHeight - 2

      - GridBeam.Y
          x: 1
          y: [0, seatDepth]
          z: seatHeight - 1

      - GridBeam.Y
          x: seatWidth - 2
          y: [0, seatDepth]
          z: seatHeight - 1
```

3d Object base: `3d-object.rimu`

```
export trait 3dObject
    (Num)


  fn translate (self, x: Num, y: Num, z: Num): Self =>
    Self
      ...self
      transform: self.transform.translate(x, y, z)
```

Stock base: `stock.rimu`

```
enum Mesh
  case Cuboid
    x_length Num
    y_length Num
    z_length Num

enum Material
  case Color (Color)

struct Renderable
  prop meshes Map<Mesh>
  prop materials Map<Material>
  prop instances List<Instance>

export trait Stock
  impl 3dObject

  fn render (self): Renderable =>
    Self
      ...self
      transform: self.transform.translate(x, y, z)
```

Example stock part: `gridbeam.rimu`

```
export struct GridBeam
  impl Stock

  prop length (Num)
    label "Length"
    description "The length of the beam in grid units"

  fn X (x: [Num, Num], y: Num, z: Num) =>
    # ...
  fn Y (x: [Num, Num], y: Num, z: Num) =>
    # ...
  fn X (x: [Num, Num], y: Num, z: Num) =>
    # ...
```

Or do we adopt the "|" (alternae) in Rhombus?

```
enum Mesh
  | Cuboid
    x_length Num
    y_length Num
    z_length Num


export struct GridBeam
  impl Stock

  prop length (Num)
    | label "Length"
    | description "The length of the beam in grid units"
```
