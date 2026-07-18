import os
import sys
import shutil
import subprocess

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

    print('\n3. Cleaning up unwanted icons...')
    cleanup_unwanted()

    print('\n4. Organizing icons into platform directories...')
    organize_icons()

    print('\n5. Syncing SVG to dist/static/icons/...')
    sync_svg_to_dist()

    print('\n' + '=' * 50)
    print('Icons built successfully!')
    print('=' * 50)

if __name__ == '__main__':
    main()
