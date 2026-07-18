import shutil
import os

icons_dir = 'f:/workspace/Slim_Transfer_2/rust/src-tauri/icons'
windows_dir = os.path.join(icons_dir, 'windows')
macos_dir = os.path.join(icons_dir, 'macos')

files = ['32x32.png', '128x128.png', '128x128@2x.png', 'icon.ico']
for f in files:
    src = os.path.join(icons_dir, f)
    dst = os.path.join(windows_dir, f)
    if os.path.exists(src):
        if os.path.exists(dst):
            os.remove(dst)
        shutil.move(src, dst)
        print(f'Moved: {f} -> windows/')

if os.path.exists(os.path.join(icons_dir, 'icon.icns')):
    src = os.path.join(icons_dir, 'icon.icns')
    dst = os.path.join(macos_dir, 'icon.icns')
    if os.path.exists(dst):
        os.remove(dst)
    shutil.move(src, dst)
    print('Moved: icon.icns -> macos/')

print('Done')