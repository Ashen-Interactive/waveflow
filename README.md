# Waveflow

**Waveflow** is a highly customizable, recursive Wave Function Collapse (WFC) tool written in Rust. It generates tileable grayscale heightmaps using rules defined in structured `.json` files, which can then be converted into 3D terrain meshes for use in game development, grayboxing, and procedural world generation.

---

## 🚀 Features

- ⚙️ **Fully customizable tile rules** via JSON
- 🔁 **Recursive field division** — define sectors that contain sub-sectors, down to the pixel level
- 🧱 Supports hierarchical field types (e.g., stone, water, mud, concrete)
- 📏 Output resolution defined by user (`X × X` tiles)
- 🌐 Ideal for prototyping levels, terrain, or custom biome structures
- 📄 Easy-to-understand config format
- 💡 Designed for extensibility (new field types, collapse rules, etc.)

---

## 🧠 How It Works

1. The user chooses an output resolution (e.g., `128 × 128`).
2. A JSON file defines the **initial fields**, their sizes, and recursive sub-structure.
3. Each field can:
   - Contain other fields
   - Define probabilities, constraints, or allowed types
   - Collapse into a final pixel value based on user rules
4. The result is a grayscale image that can be converted into a mesh or used directly in your pipeline.

---

## 🗂 JSON Example (Simplified)

```json
{
  "type": "root",
  "divide": {
    "method": "quad",
    "children": [
      {
        "type": "terrain",
        "options": ["stone", "mud", "water", "concrete"],
        "weights": [0.2, 0.3, 0.3, 0.2]
      },
      ...
    ]
  }
}
````

> The algorithm continues recursively until the field's area is smaller than a pixel, or a leaf rule is reached.

---

## 📸 Example Output

*TODO: Add screenshots of heightmaps and meshes here.*

---

## 🛠 Installation

```bash
cargo install waveflow
```

Or clone locally:

```bash
git clone https://github.com/YOUR_USERNAME/waveflow
cd waveflow
cargo run
```

---

## 📄 License

Licensed under the [Apache License 2.0](LICENSE). You are free to use this tool in personal, educational, and commercial projects — just don't claim authorship.

---

## ✨ Credits

Created by [Neo Mannskär](https://github.com/neomannskar) under the Ashen Interactive toolchain.
