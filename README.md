# Waveflow

**Waveflow** is a highly customizable, recursive Wave Function Collapse (WFC) tool written in Rust. It generates tileable grayscale heightmaps using rules defined in structured `.json` files, which can then be converted into 3D terrain meshes for use in game development, grayboxing, and procedural world generation.

---

## 🚀 Features

- ⚙️ **Recursive tile configuration** using lightweight YAML files
- 🔁 **Field-based hierarchy** with custom behavior down to the pixel
- 🧱 **Symbolic values** for fast parsing and clean design (e.g., `elevation: 0`)
- 🧩 **Tile adjacency logic** for large-scale world generation
- 🖼️ **Image output** as stitched heightmaps
- 💡 Designed for extensibility and rapid iteration

---

## 📘 How It Works

1. You define a `map.yaml` file describing the tile layout and adjacency rules.
2. Each tile is defined in its own YAML file (e.g., `ground.yaml`, `roof.yaml`), specifying recursive fields like sectors, height values, and types.
3. Waveflow reads the map, enforces constraints, and generates a stitched grayscale image that can be turned into a mesh or fed into your engine.

---

## Example: `map.yaml`

```yaml
tile_order: [ground, roof, water]

adjacency_rules:
  ground: [ground, roof, water]
  roof: [roof, ground]
  water: [ground]

tile_files:
  ground: ground.yaml
  roof: roof.yaml
  water: water.yaml
```

---

## Example `ground.yaml`

```yaml
name: Ground
elevation: 0          # Sets the ground level padding

sectors:              # Sectors are sub-areas in the heightmap
  # Sidewalk            which can contain different height values
  "1":
    elevation: "0.1"
    origin-x: "0"     # Left corner
    origin-y: "1"     # Top (origin is = Top-left)
    height: "50%",
    width: "100%",

    # What can fill this sector?
    # 'surface:'    Define later, it might load this from a different "surfaces.yaml" file
    # The 'surface' has no height data, but it might give the sector a color channel different from THE COLOR WHICH CONTROLS THE HEIGHT 
    
    # On top of, lets say 'grass' there can be a sector which is a 
    sectors:
      "1":

        origin-x: "0"
        origin-y: "1"
        height: "25%"
        width: "25%"
      "_":

  # Box/Crate
  "2":
    elevation: "0.3"
    origin-x: "1"     # Right corner
    origin-y: "0.5"   # Middle (origin is = Middle-right)
    height: "1%"
    width: "1%"
  
  "_":             # Remaining space is calculated
    elevation: "0"
```

## 📦 Installation

```bash
cargo install waveflow
```

Or clone manually:

```bash
git clone https://github.com/YOUR_USERNAME/waveflow
cd waveflow
cargo run -- --input assets/map.yaml --output out.png --size 128
```

---

## 🖼 Output

waveflow produces a grayscale PNG heightmap. You can convert this into a mesh or use it as a heightfield for terrain or logic.

---

## 🗂 File Format Goals

* Minimal and symbolic
* Easy to author by hand
* Easily versionable in Git
* Recursively nestable without performance loss

---

## 📄 License

Licensed under the [Apache License 2.0](LICENSE).
You are free to use this software in commercial and non-commercial projects.
Just don’t pretend you made it.

---

## 🧑‍💻 Credits

Created by [Neo Mannskär](https://github.com/neomannskar) and the Ashen Interactive team.

```
