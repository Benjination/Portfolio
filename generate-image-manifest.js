const fs = require('fs');
const path = require('path');

const IMAGES_DIR = path.join(__dirname, 'blog', 'images');
const MANIFEST_PATH = path.join(__dirname, 'blog', 'images.json');

function toLabel(filename) {
  const base = filename.replace(/\.[^.]+$/, '');
  return base
    .replace(/[-_]+/g, ' ')
    .replace(/\b\w/g, (m) => m.toUpperCase());
}

function run() {
  if (!fs.existsSync(IMAGES_DIR)) {
    console.error(`[images-manifest] Directory not found: ${IMAGES_DIR}`);
    fs.writeFileSync(MANIFEST_PATH, JSON.stringify([], null, 2));
    console.log(`[images-manifest] Wrote empty manifest: ${MANIFEST_PATH}`);
    return;
  }

  const exts = new Set(['.png', '.jpg', '.jpeg', '.gif', '.svg', '.webp']);
  const entries = fs
    .readdirSync(IMAGES_DIR, { withFileTypes: true })
    .filter((d) => d.isFile() && exts.has(path.extname(d.name).toLowerCase()))
    .map((d) => {
      let name = d.name;
      const full = path.join(IMAGES_DIR, name);
      try {
        const header = fs.readFileSync(full, { encoding: 'utf8', flag: 'r' }).slice(0, 200).trim();
        // If mislabeled: XML/SVG content but PNG/JPG extension, prefer .svg URL if sibling exists
        const looksSvg = header.startsWith('<?xml') || header.includes('<svg');
        if (looksSvg && !name.toLowerCase().endsWith('.svg')) {
          const svgCandidate = name.replace(/\.[^.]+$/, '.svg');
          if (fs.existsSync(path.join(IMAGES_DIR, svgCandidate))) {
            name = svgCandidate;
          }
        }
      } catch {}
      return {
        label: toLabel(name),
        path: `/blog/images/${name}`,
        file: name,
      };
    })
    .sort((a, b) => a.label.localeCompare(b.label));

  fs.writeFileSync(MANIFEST_PATH, JSON.stringify(entries, null, 2));
  console.log(`[images-manifest] Found ${entries.length} images. Wrote: ${MANIFEST_PATH}`);
}

run();
