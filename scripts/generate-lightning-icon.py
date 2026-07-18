from PIL import Image, ImageDraw

width, height = 1024, 1024
radius = 180

img = Image.new('RGBA', (width, height), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

draw.rounded_rectangle([0, 0, width, height], radius=radius, fill=(230, 126, 34, 255))

scale = width / 24
points = [
    (13 * scale, 2 * scale),
    (3 * scale, 14 * scale),
    (10 * scale, 14 * scale),
    (9 * scale, 22 * scale),
    (19 * scale, 10 * scale),
    (12 * scale, 10 * scale),
    (13 * scale, 2 * scale),
]

draw.polygon(points, fill=(255, 255, 255, 255))

img.save('f:/workspace/Slim_Transfer_2/rust/src-tauri/icons/source.png')
print(f'Generated source.png with rounded corners (radius={radius})')