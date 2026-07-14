"""Read Cloudflare OAuth token from wrangler config."""
import os

# Try different possible paths
paths = [
    os.path.expanduser("~/AppData/Roaming/xdg.config/.wrangler/config/default.toml"),
    os.path.expanduser("~/.wrangler/config/default.toml"),
    os.path.expanduser("~/AppData/Roaming/.wrangler/config/default.toml"),
]

for path in paths:
    if os.path.exists(path):
        print(f"Found config at: {path}")
        with open(path, encoding="utf-8") as f:
            content = f.read()
            print(content)
        break
else:
    # Search for any .wrangler directory
    base = os.path.expanduser("~/AppData/Roaming")
    for root, dirs, files in os.walk(base):
        if ".wrangler" in root:
            print(f"Found: {root}")
            for f in files:
                fpath = os.path.join(root, f)
                print(f"\n--- {fpath} ---")
                try:
                    with open(fpath, encoding="utf-8") as fh:
                        print(fh.read()[:500])
                except:
                    print("(binary or unreadable)")
