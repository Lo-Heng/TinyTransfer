from svglib.svglib import svg2rlg
from reportlab.graphics import renderPM

drawing = svg2rlg('f:/workspace/Slim_Transfer_2/rust/src-tauri/icons/source.svg')
renderPM.drawToFile(drawing, 'f:/workspace/Slim_Transfer_2/rust/src-tauri/icons/source.png', fmt='PNG')
print('SVG converted to PNG')