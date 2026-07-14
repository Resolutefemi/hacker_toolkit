"""Generate htool app icons with a cyberpunk lion-in-circle design."""
import os
import math
from PIL import Image, ImageDraw

def draw_lion(draw, cx, cy, size, color_green, color_cyan, color_pink):
    """Draw a cyberpunk lion head in a circle at center (cx,cy) with given size."""
    r = size / 2
    face_r = r * 0.52

    # Mane spikes
    spike_colors = [
        (color_green[0], color_green[1], color_green[2], 180),
        (color_pink[0], color_pink[1], color_pink[2], 60),
    ]
    for i, angle_deg in enumerate([0, 30, 60, 90, 120, 150, 180, 210, 240, 270, 300, 330]):
        angle = angle_deg * math.pi / 180
        inner = r * 0.68
        outer = r * 0.95
        x1 = cx + inner * math.cos(angle)
        y1 = cy + inner * math.sin(angle)
        x2 = cx + outer * math.cos(angle)
        y2 = cy + outer * math.sin(angle)
        col = spike_colors[i % 2]
        draw.line([(x1, y1), (x2, y2)], fill=col, width=max(1, int(size / 40)))

    # Mane glow triangles
    for i, angle_deg in enumerate([15, 45, 75, 105, 135, 165, 195, 225, 255, 285, 315, 345]):
        angle = angle_deg * math.pi / 180
        base_r = r * 0.8
        tip_r = r * 0.98
        bx = cx + base_r * math.cos(angle)
        by = cy + base_r * math.sin(angle)
        # Perpendicular offset
        perp = angle + math.pi / 2
        offset = r * 0.04
        tx = cx + tip_r * math.cos(angle)
        ty = cy + tip_r * math.sin(angle)
        p1x = bx + offset * math.cos(perp)
        p1y = by + offset * math.sin(perp)
        p2x = bx - offset * math.cos(perp)
        p2y = by - offset * math.sin(perp)
        draw.polygon([(p1x, p1y), (tx, ty), (p2x, p2y)],
                     fill=(color_green[0], color_green[1], color_green[2], 40))

    # Lion face circle
    draw.ellipse(
        [cx - face_r, cy - face_r * 0.9, cx + face_r, cy + face_r * 1.05],
        outline=(color_cyan[0], color_cyan[1], color_cyan[2], 200),
        width=max(1, int(size / 60)),
        fill=(12, 18, 32, 255)
    )

    # Ears
    ear_r = face_r * 0.25
    ear_offsets = [(-face_r * 0.65, -face_r * 0.7), (face_r * 0.65, -face_r * 0.7)]
    for ex, ey in ear_offsets:
        draw.ellipse(
            [cx + ex - ear_r, cy + ey - ear_r * 0.8, cx + ex + ear_r, cy + ey + ear_r * 0.8],
            outline=(color_green[0], color_green[1], color_green[2], 150),
            width=max(1, int(size / 70))
        )
        draw.ellipse(
            [cx + ex - ear_r * 0.35, cy + ey - ear_r * 0.3, cx + ex + ear_r * 0.35, cy + ey + ear_r * 0.3],
            fill=(color_cyan[0], color_cyan[1], color_cyan[2], 60)
        )

    # Eyes - cyberpunk glowing
    eye_rx = face_r * 0.22
    eye_ry = face_r * 0.18
    eye_y = cy - face_r * 0.08
    for ex_offset in [-face_r * 0.35, face_r * 0.35]:
        draw.ellipse(
            [cx + ex_offset - eye_rx, eye_y - eye_ry, cx + ex_offset + eye_rx, eye_y + eye_ry],
            outline=(color_cyan[0], color_cyan[1], color_cyan[2], 230),
            width=max(1, int(size / 80))
        )
        pupil_r = eye_rx * 0.45
        draw.ellipse(
            [cx + ex_offset - pupil_r, eye_y - pupil_r, cx + ex_offset + pupil_r, eye_y + pupil_r],
            fill=(color_cyan[0], color_cyan[1], color_cyan[2], 230)
        )
        highlight_r = pupil_r * 0.35
        draw.ellipse(
            [cx + ex_offset - highlight_r, eye_y - highlight_r, cx + ex_offset + highlight_r, eye_y + highlight_r],
            fill=(color_green[0], color_green[1], color_green[2], 255)
        )

    # Nose
    nose_size = face_r * 0.12
    nose_y = cy + face_r * 0.05
    draw.polygon(
        [(cx, nose_y - nose_size), (cx - nose_size, nose_y), (cx + nose_size, nose_y)],
        fill=(color_green[0], color_green[1], color_green[2], 200)
    )

    # Mouth
    mouth_y = cy + face_r * 0.22
    draw.arc(
        [cx - face_r * 0.2, mouth_y - face_r * 0.08, cx + face_r * 0.2, mouth_y + face_r * 0.12],
        start=0, end=180,
        fill=(color_green[0], color_green[1], color_green[2], 200),
        width=max(1, int(size / 80))
    )
    draw.line(
        [(cx, nose_y), (cx, mouth_y + face_r * 0.02)],
        fill=(color_green[0], color_green[1], color_green[2], 200),
        width=max(1, int(size / 90))
    )

    # Whiskers
    whisker_len = face_r * 0.55
    for side, sign in [(-1, -1), (1, 1)]:
        for i, wy in enumerate([-0.05, 0.05]):
            y = cy + face_r * wy
            draw.line(
                [(cx + side * face_r * 0.3, y), (cx + side * (face_r * 0.3 + whisker_len), y + side * face_r * 0.05 * (i + 1))],
                fill=(color_cyan[0], color_cyan[1], color_cyan[2], 80),
                width=max(1, int(size / 90))
            )

    # Tech marks on forehead
    mark_y = cy - face_r * 0.4
    small_r = max(1, int(size / 80))
    draw.ellipse(
        [cx - face_r * 0.1 - small_r, mark_y - small_r, cx - face_r * 0.1 + small_r, mark_y + small_r],
        fill=(color_green[0], color_green[1], color_green[2], 100)
    )
    draw.ellipse(
        [cx + face_r * 0.1 - small_r, mark_y - small_r, cx + face_r * 0.1 + small_r, mark_y + small_r],
        fill=(color_green[0], color_green[1], color_green[2], 100)
    )

    # Neon accent marks
    accent_len = face_r * 0.12
    for side in [-1, 1]:
        ay = cy
        draw.line(
            [(cx + side * face_r * 0.75, ay - accent_len), (cx + side * face_r * 0.75, ay + accent_len)],
            fill=(color_pink[0], color_pink[1], color_pink[2], 120),
            width=max(1, int(size / 80))
        )


def generate():
    os.makedirs("assets", exist_ok=True)

    # Colors
    bg_dark = (8, 12, 22, 255)
    matrix_green = (0, 255, 65)
    cyber_cyan = (10, 239, 255)
    neon_pink = (255, 0, 85)

    # Generate 1024x1024 master image
    size = 1024
    img = Image.new("RGBA", (size, size), bg_dark)
    draw = ImageDraw.Draw(img)

    # Outer circle
    margin = 20
    draw.ellipse(
        [margin, margin, size - margin - 1, size - margin - 1],
        outline=(matrix_green[0], matrix_green[1], matrix_green[2], 220),
        width=6
    )
    # Inner subtle ring
    inner_margin = margin + 30
    draw.ellipse(
        [inner_margin, inner_margin, size - inner_margin - 1, size - inner_margin - 1],
        outline=(cyber_cyan[0], cyber_cyan[1], cyber_cyan[2], 50),
        width=2
    )

    # Draw the lion
    draw_lion(draw, size // 2, size // 2, size * 0.85, matrix_green, cyber_cyan, neon_pink)

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

    # Also generate website favicon variants
    website_public = os.path.join("website", "public")
    os.makedirs(website_public, exist_ok=True)

    # Generate apple-touch-icon (180x180)
    apple = img.resize((180, 180), Image.LANCZOS)
    apple_path = os.path.join(website_public, "apple-touch-icon.png")
    apple.save(apple_path, "PNG")
    print(f"Generated {apple_path}")

    print("Done! All app icons generated with lion design.")


if __name__ == "__main__":
    generate()
