# Place the Kokoro narration lines on the film's timeline -> vo.wav + cues.json
import json, sys, numpy as np, soundfile as sf
tts = sys.argv[1]
START = {"walt": 1.4, "see": 4.2, "deals": 7.7, "games": 12.1, "drives": 16.2, "handful": 21.2,
         "imagine": 24.5, "book": 30.0, "peek": 35.3, "count": 38.7, "smart": 43.5, "phone": 50.0, "end": 55.1}
DURATION = 59.5
meta = json.load(open(f"{tts}/lines.json"))
sr = 24000
out = np.zeros(int(DURATION * sr), dtype=np.float32)
cues = []
for m in meta:
    a, r = sf.read(f"{tts}/line_{m['key']}.wav", dtype="float32"); assert r == sr
    i = int(START[m["key"]] * sr); out[i:i + len(a)] += a
    cues.append({"key": m["key"], "t0": START[m["key"]], "t1": round(START[m["key"]] + m["dur"], 3), "text": m["text"]})
for a, b in zip(cues, cues[1:]): assert a["t1"] < b["t0"], (a, b)
sf.write("vo_raw.wav", out, sr)
json.dump({"duration": DURATION, "cues": cues}, open("cues.json", "w"), indent=1)
print(json.dumps(cues[-1]))
