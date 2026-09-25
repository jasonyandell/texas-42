// Inline fonts, narration and cue sheet into film.html (a single self-contained page).
//   node build.mjs <fontsource dir>
import fs from 'fs';
const F = process.argv[2];
const b64 = p => fs.readFileSync(p).toString('base64');
const font = (pkg, file) => b64(fs.readdirSync(F).filter(d => d.startsWith(pkg) && !d.endsWith('.tgz')).map(d => `${F}/${d}/files/${file}`)[0]);
let html = fs.readFileSync('film.src.html', 'utf8')
  .replace('__CAVEAT_BRUSH__', font('fontsource-caveat-brush', 'caveat-brush-latin-400-normal.woff2'))
  .replace('__CAVEAT_700__', font('fontsource-caveat-5', 'caveat-latin-700-normal.woff2'))
  .replace('__PATRICK_HAND__', font('fontsource-patrick-hand', 'patrick-hand-latin-400-normal.woff2'))
  .replace('__CUES__', fs.readFileSync('cues.json', 'utf8').trim())
  .replace('__VO_MP3__', b64('vo.mp3'));
fs.writeFileSync('film.html', html);
console.log('film.html', (html.length / 1024).toFixed(0), 'KB');
