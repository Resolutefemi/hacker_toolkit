"""Generate htool app icons with green H and red underline design."""
import os
import math
from PIL import Image, ImageDraw, ImageFont


def generate():
    os.makedirs("assets", exist_ok=True)

    bg_dark = (8, 12, 22, 255)
    matrix_green = (0, 255, 65, 255)
    neon_pink = (255, 0, 85, 255)

    # Generate 1024x1024 master
    size = 1024
    img = Image.new("RGBA", (size, size), bg_dark)
    draw = ImageDraw.Draw(img)

    # Border rectangle with rounded corners
    margin = 20
    draw.rounded_rectangle(
        [margin, margin, size - margin - 1, size - margin - 1],
        radius=40,
        outline=matrix_green,
        width=6
    )

    # Draw "H" letter
    try:
        font = ImageFont.truetype("C:/Windows/Fonts/consola.ttf", 520)
    except (IOError, OSError):
        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf", 520)
        except (IOError, OSError):
            font = ImageFont.load_default()

    bbox = draw.textbbox((0, 0), "H", font=font)
    tw = bbox[2] - bbox[0]
    th = bbox[3] - bbox[1]
    tx = (size - tw) // 2 - bbox[0]
    ty = (size - th) // 2 - bbox[1] - 20

    # Shadow/glow for the H
    draw.text((tx + 4, ty + 4), "H", fill=(0, 255, 65, 60), font=font)
    draw.text((tx, ty), "H", fill=matrix_green, font=font)

    # Red underline
    underline_y = ty + th + 40
    line_margin = size // 6
    draw.line(
        [(line_margin, underline_y), (size - line_margin - 1, underline_y)],
        fill=neon_pink,
        width=12,
    )

    # Save PNG
    img.save("assets/icon.png")
    print(f"Generated assets/icon.png ({size}x{size})")

    # Save ICO
    ico_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    img.save("assets/icon.ico", format="ICO", sizes=ico_sizes)
    print(f"Generated assets/icon.ico with sizes {ico_sizes}")

    # Save ICNS
    icns_sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256), (512, 512)]
    img.save("assets/icon.icns", format="ICNS", sizes=icns_sizes)
    print(f"Generated assets/icon.icns with sizes {icns_sizes}")

    # Generate apple-touch-icon (180x180)
    website_public = os.path.join("website", "public")
    os.makedirs(website_public, exist_ok=True)
    apple = img.resize((180, 180), Image.LANCZOS)
    apple_path = os.path.join(website_public, "apple-touch-icon.png")
    apple.save(apple_path, "PNG")
    print(f"Generated {apple_path}")

    print("Done! All app icons generated with green H design.")


if __name__ == "__main__":
    generate()
