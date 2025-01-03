# VillageKit

## Data Model

- `Product`
  - `[PartType]`: e.g. "grid beam"
    - `[Part]`: `Transform`
      - `PartSpec`: e.g. "a grid beam of length 10"
    - `[PartRender]`
      - `[PartMeshParams]`
        - `PartMeshHandle`
      - `[PartMaterialParams]`
        - `PartMaterialHandle`
      - `[PartInstance]`: Transform

Example product: `chair.rimu`

```
def GridBeam = import("@villagekit/gridbeam")

parameters:
  seat_width:
    label: 'Seat width'
    shortId: 'sw'
    type: 'number'
    min: 5
    max: 10
    step: 5
  seat_depth:
    label: 'Seat depth'
    shortId: 'sd'
    type: 'number'
    min: 5
    max: 15
  seat_height:
    label: 'Seat height'
    description: 'The height from the ground to the top of the seat'
    shortId: 'sh'
    type: 'number'
    min: 5
    max: 15
  should_include_back:
    label: 'Include back'
    shortId: 'b'
    type: 'boolean'
  backHeight:
    label: 'Back height',
    description: 'The height from the seat to the top of the backrest'
    shortId: 'bh'
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

Example part: `gridbeam.rimu`

```
class GridBeam
  length: Num
  transform: Transform

  method X = (x: [Num, Num], y: Num, z: Num) =>
    # ...
  method Y = (x: [Num, Num], y: Num, z: Num) =>
    # ...
  method X = (x: [Num, Num], y: Num, z: Num) =>
    # ...

  method translate = (self, x: Num, y: Num, z: Num) =>
    Self
        ...self
        transform: self.transform.translate(x, y, z)
```
