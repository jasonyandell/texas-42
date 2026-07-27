export const meta = {
  name: 'adjudicate-pro-responses',
  description: 'Adversarially adjudicate ChatGPT 5.6 Pro dispatch responses: extract artifacts, execute programs, xhigh verify panel, verdict',
  phases: [
    { title: 'Extract', detail: 'parse FINAL ANSWER, programs, witnesses, proof steps' },
    { title: 'Execute', detail: 'run claimed programs sandboxed with caps' },
    { title: 'Verify', detail: 'three xhigh adversarial lenses per response' },
    { title: 'Verdict', detail: 'synthesize per-response adjudication' },
  ],
}

// args: { files: ["exchange/inbox/00N-slug.md", ...] }
const REPO = '/Users/jason/code/texas-42'
const files = (args && args.files) || []
if (!files.length) throw new Error('pass args.files: inbox paths to adjudicate')

const EXTRACT_SCHEMA = {
  type: 'object',
  required: ['finalAnswers', 'hasProgram', 'proofStepCount', 'summary'],
  properties: {
    finalAnswers: { type: 'array', items: { type: 'string' } },
    hasProgram: { type: 'boolean' },
    programPath: { type: 'string', description: 'absolute path where the extracted program was saved' },
    proofStepCount: { type: 'integer' },
    unlabeledSteps: { type: 'array', items: { type: 'integer' } },
    witnessesSaved: { type: 'string', description: 'path to extracted JSON witnesses, if any' },
    summary: { type: 'string', description: '5-sentence factual summary of what the response claims' },
    contractViolations: { type: 'array', items: { type: 'string' } },
  },
}

const EXEC_SCHEMA = {
  type: 'object',
  required: ['ran', 'outcome'],
  properties: {
    ran: { type: 'boolean' },
    outcome: { enum: ['ALL_PASS', 'SOME_FAIL', 'ERROR', 'TIMEOUT', 'NO_PROGRAM'] },
    passLines: { type: 'array', items: { type: 'string' } },
    failLines: { type: 'array', items: { type: 'string' } },
    runtimeSeconds: { type: 'number' },
    anchorsReproduced: { type: 'boolean', description: 'did it reproduce the corpus anchors it claimed (e.g. 44352165)?' },
    notes: { type: 'string' },
  },
}

const LENS_SCHEMA = {
  type: 'object',
  required: ['verdict', 'confidence', 'strongestObjection'],
  properties: {
    verdict: { enum: ['SOUND', 'FLAWED', 'UNVERIFIABLE'] },
    confidence: { enum: ['high', 'medium', 'low'] },
    strongestObjection: { type: 'string' },
    brokenSteps: { type: 'array', items: { type: 'string' } },
    salvageable: { type: 'string', description: 'what survives even if the headline fails' },
  },
}

const VERDICT_SCHEMA = {
  type: 'object',
  required: ['outcome', 'headline', 'wikiActions', 'followUpWorthAShot'],
  properties: {
    outcome: { enum: ['CONFIRMED', 'PARTIAL', 'REFUTED', 'UNVERIFIABLE', 'CONTRACT_VIOLATION'] },
    headline: { type: 'string', description: 'one sentence: what did this response actually establish?' },
    wikiActions: { type: 'array', items: { type: 'string' }, description: 'concrete wiki edits this verdict justifies' },
    followUpWorthAShot: { type: 'string', description: 'is a follow-up dispatch from the 5-shot reserve justified? What exactly? Or "no"' },
    detail: { type: 'string' },
  },
}

const LENSES = [
  ['proof-chain', 'Attack the PROOF as written: walk every numbered step and its [USES:] label; find the first gap, circularity, or scope error (finite-verified receipts cited as general theorems, deduplication semantics silently changed, canonicalization merging/missing states). The corpus in ingest/ and wiki/ is ground truth for what the cited claims actually say.'],
  ['program-vs-claim', 'Attack the PROGRAM-CLAIM correspondence: does the executed program actually compute what the FINAL ANSWER asserts, from first principles, or does it assume the answer, hardcode tables, weaken checks, or verify something subtly different (wrong dedup, wrong gauge, wrong slice)? Read the program source closely.'],
  ['corpus-consistency', 'Attack CONSISTENCY WITH THE CORPUS: check every integer, definition, and cited claim ID against ingest docs and wiki; run relevant ingest verifiers if needed; find any place the response contradicts a proved package result or silently redefines an object (support NF, reachability, outer profile).'],
]

phase('Extract')
const results = await pipeline(
  files,
  f => agent(
    `Repo: ${REPO}. Read the ChatGPT 5.6 Pro response at ${f} and its originating dispatch (exchange/outbox/ same number). Extract: every "FINAL ANSWER" line verbatim; the MACHINE-CHECKABLE ARTIFACTS python program (save it to ${REPO}/exchange/adjudication/programs/<number>.py exactly as given, creating dirs as needed); any JSON witnesses (save to ${REPO}/exchange/adjudication/witnesses/<number>.json); count the numbered proof steps and list any lacking [USES:] labels; list contract violations (missing FINAL ANSWER, non-stdlib imports, file I/O or network in the program, scientific notation integers). Do not judge correctness — extract faithfully.`,
    { label: `extract:${f.split('/').pop()}`, phase: 'Extract', schema: EXTRACT_SCHEMA },
  ),
  (ex, f) => !ex ? null : parallel([
    () => agent(
      `Repo: ${REPO}. Response ${f}; extraction summary: ${JSON.stringify(ex)}. If hasProgram, execute the saved program ${ex.programPath} with: cd ${REPO}/exchange/adjudication && timeout 2700 python3 ${ex.programPath} (background + poll; 45-minute hard cap despite the dispatch allowing 6h — if it times out, record TIMEOUT and inspect which PASS lines emerged before the cap). Capture all PASS/FAIL lines. Check specifically whether the corpus anchors the program claims to reproduce (e.g. 44,352,165) actually printed as PASS. Never modify the program. If no program, outcome NO_PROGRAM.`,
      { label: `exec:${f.split('/').pop()}`, phase: 'Execute', schema: EXEC_SCHEMA },
    ),
    ...LENSES.map(([lens, brief]) => () => agent(
      `Repo: ${REPO} (ground truth: ingest/ packages as reconciled by wiki/; read what you need). You are an adversarial referee for the external-model response at ${f} (its dispatch is in exchange/outbox/, same number). Extraction: ${JSON.stringify(ex)}. LENS — ${lens}: ${brief} Default to FLAWED/UNVERIFIABLE unless the material survives your genuine best attack; identify what survives regardless. Be specific: name steps, lines, integers.`,
      { label: `verify:${lens}:${f.split('/').pop()}`, phase: 'Verify', schema: LENS_SCHEMA, effort: 'xhigh' },
    )),
  ]),
  (panel, f, i) => !panel ? null : agent(
    `Repo: ${REPO}. Synthesize the adjudication verdict for ${f}. Execution result: ${JSON.stringify(panel[0])}. Referee reports (proof-chain, program-vs-claim, corpus-consistency): ${JSON.stringify(panel.slice(1))}. Rules: CONFIRMED needs the program green on its own claims AND no referee finding a real flaw; honest partials score PARTIAL; a single confirmed flaw in the load-bearing chain is REFUTED for the headline but record salvage; execution TIMEOUT with sound proof chain is UNVERIFIABLE-leaning-PARTIAL, say so. Recommend concrete wiki edits only for what is actually established. Reserve budget is 5 follow-up dispatches for ALL responses combined — recommend one only if it would be decisive.`,
    { label: `verdict:${f.split('/').pop()}`, phase: 'Verdict', schema: VERDICT_SCHEMA, effort: 'xhigh' },
  ).then(v => ({ file: f, verdict: v, panel })),
)

return results.filter(Boolean).map(r => ({ file: r.file, verdict: r.verdict, execution: r.panel && r.panel[0] }))
