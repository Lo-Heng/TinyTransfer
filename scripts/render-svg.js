/**
 * Render SVG to PNG using @resvg/resvg-js (pure Rust SVG renderer, no cairo needed)
 * Usage: node render-svg.js <input.svg> <output.png> [size]
 */
const { Resvg } = require('@resvg/resvg-js');
const fs = require('fs');
const path = require('path');

const args = process.argv.slice(2);
if (args.length < 2) {
  console.error('Usage: node render-svg.js <input.svg> <output.png> [size]');
  process.exit(1);
}

const input = args[0];
const output = args[1];
const size = parseInt(args[2] || '1024', 10);

const svg = fs.readFileSync(input);
const resvg = new Resvg(svg, {
  fitTo: { mode: 'width', value: size }
});
const pngData = resvg.render().asPng();
fs.writeFileSync(output, pngData);

console.log(`Rendered ${input} -> ${output} (${size}x${size})`);
