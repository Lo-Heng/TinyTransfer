import svgwrite

dwg = svgwrite.Drawing(
    'f:/workspace/Slim_Transfer_2/rust/src-tauri/icons/source.svg',
    size=(1024, 1024),
    viewBox="0 0 1024 1024"
)

dwg.add(dwg.rect(
    insert=(0, 0),
    size=(1024, 1024),
    fill="#E67E22"
))

scale = 1024 / 24
path_data = "M13 2L3 14h7l-1 8 10-12h-7l1-8z"

scaled_path = []
parts = []
current = []
i = 0
while i < len(path_data):
    c = path_data[i]
    if c in 'MLHVCSQTAZmlhvcsqtaz':
        if current:
            parts.append((parts[-1][0] if parts else c, current))
        current = []
        parts.append((c, []))
        i += 1
    elif c == ' ':
        i += 1
    else:
        j = i
        while j < len(path_data) and path_data[j] not in ' MLHVCSQTAZmlhvcsqtaz':
            j += 1
        val = float(path_data[i:j])
        current.append(val)
        i = j

transformed_parts = []
for cmd, coords in parts:
    if cmd in 'ML':
        new_coords = []
        for k in range(0, len(coords), 2):
            new_coords.append(coords[k] * scale)
            new_coords.append(coords[k+1] * scale)
        transformed_parts.append((cmd, new_coords))
    elif cmd in 'HV':
        transformed_parts.append((cmd, [v * scale for v in coords]))
    else:
        transformed_parts.append((cmd, coords))

final_path = ""
for cmd, coords in transformed_parts:
    final_path += cmd
    for i, v in enumerate(coords):
        if i > 0:
            final_path += " "
        final_path += str(v)

dwg.add(dwg.path(
    d=final_path,
    fill="#FFFFFF"
))

dwg.save()
print('Generated source.svg')