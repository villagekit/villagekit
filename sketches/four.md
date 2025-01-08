Add `of` keyword to improve types of types.

Base math: `math.rimu`

```
export
  Vector3
  Quaternion

struct Vector3
  input x: Num
    default = 0

  input y: Num
    default = 0

  input z: Num
    default = 0

  fn length
    input self
    output Num
  do
    let { x, y, z } = self
    sqrt(x * x, y * y, z * z)

struct Quaternion
  input x: Num
    default = 0

  input y: Num
    default = 0

  input z: Num
    default = 0

  input z: Num
    default = 1
```

3d object trait: `object-3d.rimu`

```
import
  from @std/math
    Vector3
    Quaternion

export Object3d

struct Transform
  input translation: Vector3

  input rotation: Quaternion

  input scale: Vector3

  fn translate
    input self
    input x: Num
    input y: Num
    input z: Num
    output Self
  do
    Self
      ...self
      translation = Vector3
        self.translation.x + x
        self.translation.y + y
        self.translation.z + z

trait Object3d
  input transform: Transform

  fn translate
    input self
    input Num
    input Num
    input Num
    output Self
  do
    Self
      ...self
      transform: self.transform.translate(x, y, z)
```

Assembly trait: `assembly.rimu`

```
import
  Object3d = @std/object-3d

export Assembly

type Part = Union
  of Stock
  of Assembly

type Parts = List
  meta default = []

  of Union
    of Parts
    of Part
    of Null

trait Assembly
  impl Object3d

  fn parts: Parts
```


Example assembly: `chair.rimu`

```
import
  Assembly = @std/assembly@1
  GridBeam = @villagekit/gridbeam@1
  SmartFasteners = @villagekit/smart-fasteners@1

export Chair

struct Chair
  input seat_width: Num
    label = 'Seat width'
    min = 5
    max = 10
    step = 5

  input seat_depth: Num
    label = 'Seat depth'
    min = 5
    max = 15

  input seat_height: Num
    label = 'Seat height'
    description = 'The height from the ground to the top of the seat'
    min = 5
    max = 15

  input should_include_back: Bool
    label 'Include back'

  input back_height: Num
    label = 'Back height'
    description = 'The height from the seat to the top of the backrest'
    if = fn (self) =>
      self.should_include_back
    min = 5
    max = 10

  fn regular(): Self =>
    meta label = 'Regular (Without Back)'

    Self
      back_height = 10
      seat_depth = 10
      seat_height = 10
      seat_width = 10
      should_include_back = false

  fn regular_with_back(): Self =>
    meta label = 'Regular With Back'

    Self
      ...Self.regular()
      should_include_back = true

  const plugins = [SmartFasteners()]

  impl Assembly
    fn parts (self) =>
      let from self = Map
        seat_width
        seat_depth
        seat_height
        back_height
        should_include_back

      let back_z_beam_end_z = if should_include_back then seat_height + back_height else seat_height
      let seat_panel_start_y = if should_include_back then -1 else 0
      let seat_panel_end_y = if should_include_back then seat_depth - 1 else seat_depth

      List
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

Renderable struct `renderable.rimu`

```
let export = Renderable

enum Mesh
  case Cuboid
    x_length: Num
    y_length: Num
    z_length: Num

enum Material
  case Color
    color: Color

struct Instance
  prop mesh: Str
  prop material: Str
  prop transform: Transform
  prop children: List(Self)

struct Renderable
  prop meshes: Map(Mesh)
  prop materials: Map(Material)
  prop instances: List(Instance)
```

Stock trait: `stock.rimu`

```
let Object3d = import("@std/object-3d")

let export = Stock

trait Stock: Object3d
  fn render (self): Renderable
```

Example stock part: `gridbeam.rimu`

```
let Stock = import("@std/stock")

let export = GridBeam

struct GridBeam
  input length: Num
    label = "Length"
    description = "The length of the beam in grid units"

  fn X
    input x: [Num, Num]
    input y: Num
    input z: Num
    output Self
  =>
    pipe
      Self({ length: abs(x.1 - x.0) })
      .rotate(
    # ...

  fn Y (x: Num, y: [Num, Num], z: Num): Self =>
    # ...

  fn Z (x: Num, y: Num, z: [Num, Num]): Self =>
    # ...

  impl Stock
    fn render
      input self
      output Rendable
    do
      # ...
```
