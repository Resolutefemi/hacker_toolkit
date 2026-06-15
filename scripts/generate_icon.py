import os
import random
from PIL import Image, ImageDraw

def generate():
    # Make sure assets directory exists
    os.makedirs("assets", exist_ok=True)
    
    # 1024x1024 image
    size = 1024
    img = Image.new("RGBA", (size, size), (13, 15, 18, 255)) # Sleek dark background (#0D0F12)
    draw = ImageDraw.Draw(img)
    
    # Draw futuristic elements
    # Shield shape points
    shield_pts = [
        (512, 150),  # Top center
        (800, 250),  # Top right
        (800, 600),  # Mid right
        (512, 880),  # Bottom center
        (224, 600),  # Mid left
        (224, 250),  # Top left
    ]
    
    # Outer shield glow (multiple widths)
    for i in range(15, 0, -2):
        alpha = int(80 / i)
        draw.polygon(shield_pts, outline=(0, 255, 0, alpha), width=i)
        
    # Draw solid green neon outline
    draw.polygon(shield_pts, outline=(0, 255, 0, 255), width=6)
    
    # Draw crosshair or target lines
    draw.line((100, 512, 924, 512), fill=(0, 255, 0, 40), width=2)
    draw.line((512, 100, 512, 924), fill=(0, 255, 0, 40), width=2)
    draw.ellipse((512-300, 512-300, 512+300, 512+300), outline=(0, 255, 0, 20), width=2)
    draw.ellipse((512-400, 512-400, 512+400, 512+400), outline=(0, 255, 0, 10), width=2)
    
    # Draw terminal binary patterns in background (simulating matrix)
    random.seed(42)
    # Draw small matrix dots/shapes
    for _ in range(250):
        x = random.randint(250, 770)
        y = random.randint(200, 800)
        # Check if point is inside shield using simple bounding box/shield approximation
        if 200 < y < 800 and 224 < x < 800:
            val = random.choice(["0", "1", ".", "+", "#"])
            # Draw simple pixelated characters using lines
            if val == "1":
                draw.line((x, y-6, x, y+6), fill=(0, 255, 0, 70), width=2)
            elif val == "0":
                draw.ellipse((x-5, y-5, x+5, y+5), outline=(0, 255, 0, 50), width=1)
            else:
                draw.ellipse((x-2, y-2, x+2, y+2), fill=(0, 255, 0, 60))
                
    # Draw glowing Padlock in center
    # Shackle (curve)
    draw.arc((512-80, 512-160, 512+80, 512), start=180, end=360, fill=(0, 255, 0, 255), width=20)
    draw.line((512-80, 512-80, 512-80, 512), fill=(0, 255, 0, 255), width=20)
    draw.line((512+80, 512-80, 512+80, 512), fill=(0, 255, 0, 255), width=20)
    
    # Padlock Body
    draw.rounded_rectangle((512-120, 512-30, 512+120, 512+150), radius=20, fill=(13, 15, 18, 255), outline=(0, 255, 0, 255), width=8)
    
    # Keyhole
    draw.ellipse((512-25, 512+20, 512+25, 512+70), fill=(0, 255, 0, 255))
    draw.polygon([(512-15, 512+50), (512+15, 512+50), (512+20, 512+100), (512-20, 512+100)], fill=(0, 255, 0, 255))

    # Save PNG
    img.save("assets/icon.png")
    print("Generated assets/icon.png")
    
    # Save ICO
    img.save("assets/icon.ico", format="ICO", sizes=[(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])
    print("Generated assets/icon.ico")
    
    # Save ICNS
    img.save("assets/icon.icns", format="ICNS", sizes=[(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256), (512, 512)])
    print("Generated assets/icon.icns")

if __name__ == "__main__":
    generate()
