# Place the Kokoro narration lines on the film's timeline -> vo.wav + cues.json
import json, sys, numpy as np, soundfile as sf
tts = sys.argv[1]
START = {"walt": 1.4, "see": 4.2, "how": 7.45, "deals": 10.0, "deals2": 14.5, "games": 17.15,
         "drives": 22.0, "dollars": 25.9, "onemove": 29.2, "handful": 31.2, "imagine": 38.9, "book": 44.4,
         "peek": 49.7, "count": 53.1, "smart": 59.15, "recap": 66.6, "phone": 74.55, "end": 79.65}
DURATION = 84.05
meta = json.load(open(f"{tts}/lines.json"))
sr = 24000
out = np.zeros(int(DURATION * sr), dtype=np.float32)
cues = []
for m in meta:
    a, r = sf.read(f"{tts}/line_{m['key']}.wav", dtype="float32"); assert r == sr
    i = int(START[m["key"]] * sr); out[i:i + len(a)] += a
    cues.append({"key": m["key"], "t0": START[m["key"]], "t1": round(START[m["key"]] + m["dur"], 3), "text": m["text"]})
cues.sort(key=lambda c: c["t0"])
for a, b in zip(cues, cues[1:]): assert a["t1"] < b["t0"], (a, b)
sf.write("vo_raw.wav", out, sr)
json.dump({"duration": DURATION, "cues": cues}, open("cues.json", "w"), indent=1)
print(json.dumps(cues[-1]))
