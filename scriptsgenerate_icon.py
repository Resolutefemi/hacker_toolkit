import struct
import sys
from PIL import Image, ImageDraw, ImageFont

def create_icon():
    # Create a 256x256 cyberpunk-style icon
    sizes = [256, 128, 64, 48, 32, 16]
    
    for size in sizes:
        img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)
        
        # Dark background with rounded corners
        margin = size // 12
        draw.rounded_rectangle(
            [margin, margin, size - margin - 1, size - margin - 1],
            radius=size // 6,
            fill=(8, 12, 22, 255),
            outline=(0, 255, 65, 200),
            width=max(1, size // 32)
        )
        
        # Inner glow
        inner_margin = size // 4
        draw.rounded_rectangle(
            [inner_margin, inner_margin, size - inner_margin - 1, size - inner_margin - 1],
            radius=size // 8,
            fill=(0, 255, 65, 20),
            outline=(10, 239, 255, 100),
            width=max(1, size // 48)
        )
        
        # Lightning bolt "H" symbol
        cx, cy = size // 2, size // 2
        bolt_size = size // 4
        
        # Draw hacker-style "H"
        h_char = "H"
        try:
            font_size = size // 2
            font = ImageFont.truetype("arial.ttf", font_size)
        except:
            font = ImageFont.load_default()
        
        # Center the letter
        bbox = draw.textbbox((0, 0), h_char, font=font)
        tw, th = bbox[2] - bbox[0], bbox[3] - bbox[1]
        tx = (size - tw) // 2 - bbox[0]
        ty = (size - th) // 2 - bbox[1]
        
        # Draw with glow
        for offset in [(2,0),(-2,0),(0,2),(0,-2)]:
            draw.text((tx+offset[0], ty+offset[1]), h_char, fill=(10, 239, 255, 60), font=font)
        draw.text((tx, ty), h_char, fill=(0, 255, 65, 255), font=font)
        
        # Bottom accent line
        line_y = size - margin * 2
        draw.rectangle(
            [margin * 3, line_y, size - margin * 3, line_y + max(1, size // 48)],
            fill=(255, 0, 85, 200)
        )
        
        # Save as PNG first
        filename = f"assets/icon_{size}.png"
        img.save(filename)
        print(f"Created {filename}")
    
    print("Icons generated successfully!")

if __name__ == "__main__":
    create_icon()
