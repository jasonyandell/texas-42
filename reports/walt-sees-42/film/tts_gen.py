import json, sys, soundfile as sf, numpy as np
from kokoro_onnx import Kokoro
k = Kokoro("kokoro-v1.0.onnx", "voices-v1.0.bin")
lines = json.load(open(sys.argv[1]))
voice = sys.argv[2]; speed = float(sys.argv[3])
meta = []
for i, (key, text) in enumerate(lines):
    a, sr = k.create(text, voice=voice, speed=speed, lang="en-us")
    # trim leading/trailing near-silence
    idx = np.where(np.abs(a) > 0.01)[0]
    a = a[max(0, idx[0]-600): idx[-1]+2400]
    sf.write(f"line_{key}.wav", a, sr)
    meta.append({"key": key, "text": text, "dur": round(len(a)/sr, 3)})
    print(key, round(len(a)/sr, 2), text)
json.dump(meta, open("lines.json", "w"), indent=1)
