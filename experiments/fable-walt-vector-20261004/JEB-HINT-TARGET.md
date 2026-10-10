# Jason's exact hint target, screenshot clarified during source inspection

Parent visually inspected Jason's screenshot, Library `libfile_aad74ba33dc8819199c4c61b2195a7d6`, filename `1000004657.png`. No image is materialized here; the following visible text was forwarded by the coordinating parent:

- Title: **Estimated chance to make the bid**
- Legal rows: `2:0 100.0% 40/40 Suggested Highest estimate`; `2:1 100.0% 40/40`; `3:2 100.0% 40/40`; `5:0 100.0% 40/40`; `3:0 95.0% 38/40`.
- Footer: **Hints use your hand and public plays. They compare the baseline player's outcomes without the separate partner check.**

Jason: **these numbers are the things to train against. the little model should say what Walt would say at that level.**

This specifies the actual HINT per-action make-bid vector at the named level, including numerator/denominator and provenance. It does not automatically specify the live move selector's final objective if that differs. Trace hint UI -> API -> exact Walt source, identifying the omission of the separate partner check. Retain the entire vector and ties, legal mask, sample counts, level and seed/teacher/source versions. Do not treat 40/40 as guaranteed win or exact equality of latent chances. This supersedes any plan that substitutes a live-move finalobjective for the actual hint numbers. No changes to the completed frozen diagnosis, no tuning on its test. Plan/source mapping first, before heavy new runs.
