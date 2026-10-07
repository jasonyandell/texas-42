"""Assemble walt-explained.html from template.html, engine.js and data.json.

Run from this directory: `node pre.js && python3 build.py`.
"""
import json
import re

engine = open("engine.js").read()
engine = engine.replace("(function (root) {", "function WaltLiteFactory() {", 1)
engine = engine.replace(
    "if (typeof module !== 'undefined') module.exports = api; else root.WaltLite = api;", "return api;", 1
)
engine = re.sub(r"\}\)\(this\);\s*$", "}\n", engine)
data = json.dumps(json.load(open("data.json")), separators=(",", ":"))
page = open("template.html").read().replace("/*ENGINE*/", engine).replace("/*DATA*/", data)
open("walt-explained.html", "w").write(page)
