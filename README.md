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
import
  Assembly = @std/assembly@1
  GridBeam = @villagekit/gridbeam@1
  SmartFasteners = @villagekit/smart-fasteners@1

export = Chair

struct Chair: Assembly
  prop seat_width: Num
    label 'Seat width'
    min 5
    max 10
    step 5

  prop seat_depth: Num
    label 'Seat depth'
    min 5
    max 15

  prop seat_height: Num
    label 'Seat height'
    description 'The height from the ground to the top of the seat'
    min 5
    max 15

  prop should_include_back: Bool
    label 'Include back'

  prop back_height: Num
    label 'Back height',
    description 'The height from the seat to the top of the backrest'
    min 5
    max 10

  preset regular
    label 'Regular (Without Back)'
    values
      back_height = 10
      seat_depth = 10
      seat_height = 10
      seat_width = 10
      should_include_back = false

  preset regular_with_back
    label 'Regular With Back'
    values
      back_height = 10
      seat_depth = 10
      seat_height = 10
      seat_width = 10
      should_include_back = true

  plugins = [SmartFasteners()]

  fn parts (self): List<Product> =>
    let object self =
      seat_width =
      seat_depth =
      seat_height =
      back_height =
      should_include_back =

    let back_z_beam_end_z = if should_include_back then seat_height + back_height else seat_height
    let seat_panel_start_y = if should_include_back then -1 else 0
    let seat_panel_end_y = if should_include_back then seat_depth - 1 else seat_depth

    list
      GridPanel.XY
        x: [0, seat_width]
        y: [seat_panel_start_y, seat_panel_end_y]
        z: seatHeight

      if should_include_back
        GridPanel.XZ
          x: [0, seat_width]
          y: seat_depth - 2
          z: [seat_height + 1, seat_height + 1 + back_height]
          fit: 'top'

      GridBeam.Z
        x: 0
        y: 0
        z: [0, seat_height]

      GridBeam.Z
        x: seat_width - 1
        y: 0
        z: [0, seat_height]

      GridBeam.Z
        x: 0
        y: seat_depth - 1
        z: [0, back_z_beam_end_z]

      GridBeam.Z
        x: seat_width - 1
        y: seat_depth - 1
        z: [0, back_z_beam_end_z]

      GridBeam.X
        x: [0, seat_width]
        y: 1
        z: seat_height - 2

      GridBeam.X
        x: [0, seat_width]
        y: seat_depth - 2
        z: seat_height - 2

      GridBeam.Y
        x: 1
        y: [0, seat_depth]
        z: seat_height - 1

      GridBeam.Y
        x: seat_width - 2
        y: [0, seat_depth]
        z: seat_height - 1
```

3d Object base: `3d-object.rimu`

```
export trait Object3d
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
    x_length: Num
    y_length: Num
    z_length: Num

enum Material
  case Color(Color)

struct Renderable
  prop meshes: Map<Mesh>
  prop materials: Map<Material>
  prop instances: List<Instance>

export trait Stock
  impl Object3d

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

  fn X (x: [Num, Num], y: Num, z: Num): Self =>
    # ...
  fn Y (x: [Num, Num], y: Num, z: Num): Self =>
    # ...
  fn X (x: [Num, Num], y: Num, z: Num): Self =>
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

Open questions:

- Should we adopt more of Rhombus?
  - `def`
  - `|` alternate
  - `namespace`
  - Shubbery notation: https://docs.racket-lang.org/shrubbery/index.html
- Should we adopt more of Rust?
  - traits and structs
  - traits for core functionality
    - e.g. `Add`, `Sub`, `Mul`, `Div`, etc
