# How Walt sees 42 — the one-minute film

A hand-drawn animated short (≈84 s) about Walt's sampling idea, for friends and family.
`film.html` is the whole thing: pictures drawn by canvas JavaScript, score and foley synthesized
in the page, narration embedded. EXPLORATORY teaching material, not a result: 399,072,960 is exact;
"billions of ways", "ten million hard drives" and "three billion dollars" are estimates (exact play-out counts for 12 random
deals ranged ~3×10⁸ to 3×10¹¹); the tallies on screen are made up for the cartoon.

Rebuild:

1. Narration — [Kokoro](https://github.com/thewh1teagle/kokoro-onnx) (`kokoro-v1.0.onnx`, `voices-v1.0.bin`
   from its GitHub release; `pip install kokoro-onnx soundfile`), voice `af_heart`:
   `python3 tts_gen.py narration.json af_heart 1.0` (in the model dir), then
   `python3 assemble_vo.py <model dir>` → `vo_raw.wav`, `cues.json`; normalise to `vo.wav`/`vo.mp3`
   (ffmpeg `highpass=f=70,acompressor,loudnorm=I=-16`).
2. Page — `node build.mjs <dir of @fontsource packages>` inlines Caveat Brush, Caveat 700,
   Patrick Hand, `cues.json` and `vo.mp3` into `film.src.html` → `film.html`.
3. Video — `node export.mjs audio` writes the page's own synth to `fx.wav`;
   `node export.mjs video 30 <from> <to> <out>` renders frames (run segments in parallel, concat),
   then mix `fx.wav` + `vo.wav` (amix, alimiter) and encode (x264 crf 23, AAC 192k).
   `node stills.mjs <dir> 3.6,10.5,...` renders spot-check frames.

Playwright's Chromium and an ffmpeg with libx264 are assumed (`CHROMIUM`, `FFMPEG`, `NODE_PATH`).
