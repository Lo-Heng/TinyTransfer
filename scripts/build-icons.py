import os
import sys
import shutil
import subprocess
from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SVG_SRC = os.path.join(ROOT, 'frontend', 'public', 'static', 'icons', 'lightning.svg')
RENDER_JS = os.path.join(ROOT, 'scripts', 'render-svg.js')
ICONS_DIR = os.path.join(ROOT, 'rust', 'src-tauri', 'icons')
SOURCE_PNG = os.path.join(ICONS_DIR, 'source.png')
DIST_ICONS_DIR = os.path.join(ROOT, 'rust', 'src-tauri', 'dist', 'static', 'icons')

def generate_source_png():
    """Use @resvg/resvg-js to render SVG to PNG (preserves exact SVG rendering)"""
    result = subprocess.run(
        ['node', RENDER_JS, SVG_SRC, SOURCE_PNG, '1024'],
        cwd=ROOT,
        capture_output=True,
        text=True,
        shell=True
    )
    print(result.stdout.strip())
    if result.returncode != 0:
        print(result.stderr)
        return False
    return True

def run_tauri_icon():
    cwd = os.path.join(ROOT, 'rust', 'src-tauri')
    cmd = f'npx @tauri-apps/cli icon icons/source.png'
    result = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, shell=True)
    print(result.stdout)
    if result.stderr:
        print(result.stderr)
    return result.returncode == 0

def generate_proper_ico():
    import struct
    sizes = [16, 24, 32, 48, 64, 128, 256]
    src = Image.open(SOURCE_PNG).convert('RGBA')
    
    icon_entries = []
    icon_data = b''
    data_offset = 6 + len(sizes) * 16
    
    for size in sizes:
        img = src.resize((size, size), Image.LANCZOS)
        pixels = img.load()
        
        xor_data = bytearray()
        and_data = bytearray()
        row_pad = (4 - ((size * 4) % 4)) % 4
        and_row_bytes = (size + 31) // 32 * 4
        
        for y in range(size - 1, -1, -1):
            for x in range(size):
                r, g, b, a = pixels[x, y]
                xor_data.extend([b, g, r, a])
            for _ in range(row_pad):
                xor_data.append(0)
        
        for y in range(size - 1, -1, -1):
            for byte_idx in range(and_row_bytes):
                byte_val = 0
                for bit_idx in range(8):
                    pixel_idx = byte_idx * 8 + bit_idx
                    if pixel_idx < size:
                        _, _, _, a = pixels[pixel_idx, y]
                        if a < 128:
                            byte_val |= (1 << (7 - bit_idx))
                and_data.append(byte_val)
        
        biSize = 40
        biWidth = size
        biHeight = size * 2
        biPlanes = 1
        biBitCount = 32
        biCompression = 0
        biSizeImage = len(xor_data) + len(and_data)
        
        bmp_header = struct.pack('<IiiHHIIiiII',
            biSize, biWidth, biHeight, biPlanes, biBitCount,
            biCompression, biSizeImage, 0, 0, 0, 0)
        
        icon_img_data = bmp_header + bytes(xor_data) + bytes(and_data)
        
        w = size if size < 256 else 0
        h = size if size < 256 else 0
        entry = struct.pack('<BBBBHHII',
            w, h, 0, 0, 1, 32,
            len(icon_img_data), data_offset)
        
        icon_entries.append(entry)
        data_offset += len(icon_img_data)
        icon_data += icon_img_data
    
    header = struct.pack('<HHH', 0, 1, len(sizes))
    
    ico_path = os.path.join(ICONS_DIR, 'icon.ico')
    with open(ico_path, 'wb') as f:
        f.write(header)
        for entry in icon_entries:
            f.write(entry)
        f.write(icon_data)
    
    print(f'Generated proper ICO with BMP+AND mask: {ico_path}')
    return True

def cleanup_unwanted():
    unwanted = ['Square30x30Logo.png', 'Square44x44Logo.png', 'Square71x71Logo.png',
                'Square89x89Logo.png', 'Square107x107Logo.png', 'Square142x142Logo.png',
                'Square150x150Logo.png', 'Square284x284Logo.png', 'Square310x310Logo.png',
                'StoreLogo.png', '64x64.png', 'icon.png']
    for f in unwanted:
        path = os.path.join(ICONS_DIR, f)
        if os.path.exists(path):
            os.remove(path)
            print(f'Removed: {f}')

def organize_icons():
    windows_dir = os.path.join(ICONS_DIR, 'windows')
    macos_dir = os.path.join(ICONS_DIR, 'macos')

    if not os.path.exists(windows_dir):
        os.makedirs(windows_dir)
    if not os.path.exists(macos_dir):
        os.makedirs(macos_dir)

    windows_files = ['32x32.png', '128x128.png', '128x128@2x.png', 'icon.ico']
    for f in windows_files:
        src = os.path.join(ICONS_DIR, f)
        dst = os.path.join(windows_dir, f)
        if os.path.exists(src):
            if os.path.exists(dst):
                os.remove(dst)
            shutil.move(src, dst)
            print(f'Moved: {f} -> windows/')

    if os.path.exists(os.path.join(ICONS_DIR, 'icon.icns')):
        src = os.path.join(ICONS_DIR, 'icon.icns')
        dst = os.path.join(macos_dir, 'icon.icns')
        if os.path.exists(dst):
            os.remove(dst)
        shutil.move(src, dst)
        print('Moved: icon.icns -> macos/')

def sync_svg_to_dist():
    """Copy SVG source to dist/static/icons/ so Tauri app can serve it"""
    if not os.path.exists(DIST_ICONS_DIR):
        os.makedirs(DIST_ICONS_DIR)
    dst = os.path.join(DIST_ICONS_DIR, 'lightning.svg')
    shutil.copy2(SVG_SRC, dst)
    print(f'Copied SVG to {dst}')

def main():
    print('=' * 50)
    print('Building icons from SVG source')
    print('=' * 50)

    if not os.path.exists(SVG_SRC):
        print(f'Error: SVG source not found at {SVG_SRC}')
        sys.exit(1)

    print('\n1. Rendering SVG to source.png via resvg...')
    if not generate_source_png():
        print('Error: SVG rendering failed')
        sys.exit(1)

    print('\n2. Running tauri icon command...')
    if not run_tauri_icon():
        print('Error: tauri icon command failed')
        sys.exit(1)

    print('\n3. Generating proper ICO (BMP + AND mask)...')
    if not generate_proper_ico():
        print('Warning: proper ICO generation failed, using tauri default')

    print('\n4. Cleaning up unwanted icons...')
    cleanup_unwanted()

    print('\n5. Organizing icons into platform directories...')
    organize_icons()

    print('\n6. Syncing SVG to dist/static/icons/...')
    sync_svg_to_dist()

    print('\n' + '=' * 50)
    print('Icons built successfully!')
    print('=' * 50)

if __name__ == '__main__':
    main()
