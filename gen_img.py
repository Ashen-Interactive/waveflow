import sys
import os
from PIL import Image

def hex_to_rgb(hex_color):
    hex_color = hex_color.lstrip('#')
    if len(hex_color) == 3:
        hex_color = ''.join([c*2 for c in hex_color])
    if len(hex_color) != 6:
        raise ValueError("Invalid hex color format.")
    return tuple(int(hex_color[i:i+2], 16) for i in (0, 2, 4))

def main():
    if len(sys.argv) != 2:
        print("Usage: python gen_img.py <hex_color>")
        sys.exit(1)
    hex_color = sys.argv[1]
    try:
        rgb = hex_to_rgb(hex_color)
    except ValueError as e:
        print(f"Error: {e}")
        sys.exit(1)

    img = Image.new('RGB', (512, 512), rgb)
    out_dir = os.path.join(os.path.dirname(__file__), 'example', 'tiles')
    os.makedirs(out_dir, exist_ok=True)
    filename = f"tile_{hex_color.lstrip('#')}.png"
    out_path = os.path.join(out_dir, filename)
    img.save(out_path)
    print(f"Image saved to {out_path}")

if __name__ == "__main__":
    main()