"""Generate PNG assets for the htool download website."""
from PIL import Image, ImageDraw, ImageFont
import os
import sys

def create_og_image():
    """Create 1200x630 OG image for social sharing."""
    img = Image.new('RGBA', (1200, 630), (8, 12, 22, 255))
    draw = ImageDraw.Draw(img)

    # Cyberpunk border
    draw.rectangle([10, 10, 1190, 620], outline=(0, 255, 65, 200), width=3)
    draw.rectangle([18, 18, 1182, 612], outline=(10, 239, 255, 100), width=1)

    # Corner accents
    accent_len = 60
    for (x, y, dx, dy) in [
        (18, 18, accent_len, 0), (18, 18, 0, accent_len),
        (1182, 18, -accent_len, 0), (1182, 18, 0, accent_len),
        (18, 612, accent_len, 0), (18, 612, 0, -accent_len),
        (1182, 612, -accent_len, 0), (1182, 612, 0, -accent_len),
    ]:
        draw.line([(x, y), (x + dx, y + dy)], fill=(10, 239, 255, 180), width=3)

    # Huge "htool" text
    center_x, center_y = 600, 250
    try:
        font_large = ImageFont.truetype("C:/Windows/Fonts/consola.ttf", 120)
        font_small = ImageFont.truetype("C:/Windows/Fonts/consola.ttf", 36)
    except (IOError, OSError):
        try:
            font_large = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf", 120)
            font_small = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf", 36)
        except (IOError, OSError):
            font_large = ImageFont.load_default()
            font_small = ImageFont.load_default()

    # Draw "htool" with glitch effect
    draw.text((center_x - 180 + 3, center_y - 60 + 3), "htool", fill=(255, 0, 85, 150), font=font_large)
    draw.text((center_x - 180 - 2, center_y - 60 - 2), "htool", fill=(10, 239, 255, 150), font=font_large)
    draw.text((center_x - 180, center_y - 60), "htool", fill=(0, 255, 65, 255), font=font_large)

    # Subtitle
    draw.text((center_x - 200, center_y + 80), "Cybersecurity Toolkit", fill=(200, 215, 230, 255), font=font_small)
    draw.text((center_x - 170, center_y + 130), "by Resolute Femi", fill=(100, 120, 140, 200), font=font_small)

    # Bottom scan line effect
    for i in range(3):
        y = 570 + i * 12
        draw.rectangle([50, y, 1150, y + 2], fill=(0, 255, 65, 40 + i * 20))

    return img

def create_apple_touch_icon():
    """Create 180x180 apple touch icon."""
    size = 180
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Dark rounded rect background
    margin = 6
    draw.rounded_rectangle(
        [margin, margin, size - margin - 1, size - margin - 1],
        radius=24, fill=(8, 12, 22, 255),
        outline=(0, 255, 65, 200), width=3
    )

    # Inner glow
    inner = margin + 20
    draw.rounded_rectangle(
        [inner, inner, size - inner - 1, size - inner - 1],
        radius=12, outline=(10, 239, 255, 80), width=2
    )

    # "H" letter
    try:
        font = ImageFont.truetype("C:/Windows/Fonts/consola.ttf", 100)
    except (IOError, OSError):
        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf", 100)
        except (IOError, OSError):
            font = ImageFont.load_default()

    bbox = draw.textbbox((0, 0), "H", font=font)
    tw = bbox[2] - bbox[0]
    th = bbox[3] - bbox[1]
    tx = (size - tw) // 2 - bbox[0]
    ty = (size - th) // 2 - bbox[1]

    # Glow effect
    draw.text((tx + 2, ty + 2), "H", fill=(255, 0, 85, 120), font=font)
    draw.text((tx - 1, ty - 1), "H", fill=(10, 239, 255, 120), font=font)
    draw.text((tx, ty), "H", fill=(0, 255, 65, 255), font=font)

    # Bottom accent line
    line_y = size - 24
    draw.rectangle([size//3, line_y, 2*size//3, line_y + 3], fill=(255, 0, 85, 200))

    return img

def main():
    out_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'website', 'public')

    os.makedirs(out_dir, exist_ok=True)

    # OG Image
    og = create_og_image()
    og_path = os.path.join(out_dir, 'og-image.png')
    og.save(og_path, 'PNG')
    print(f"Created {og_path} ({og.size[0]}x{og.size[1]})")

    # Apple Touch Icon
    ati = create_apple_touch_icon()
    ati_path = os.path.join(out_dir, 'apple-touch-icon.png')
    ati.save(ati_path, 'PNG')
    print(f"Created {ati_path} ({ati.size[0]}x{ati.size[1]})")

    print("Done! All website assets generated.")

if __name__ == '__main__':
    main()
