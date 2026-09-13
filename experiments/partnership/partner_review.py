"""Optional count-offer investigation, with the completed L1 as backstop.

The cheap public detector is a specialization of the checked-in Scheme query.
Native parity is checked before a completed investigation may change the move.
Timeout/refusal is unresolved, never evidence that the offer was bad or absent.
"""
from pathlib import Path

from rules import TILES, winner

BINARY = Path(__file__).resolve().parents[2] / "walt/target/release/partner_review"
ID = "partner-count-review-v1"
MAX_SECONDS = 0.250
MAX_WORLDS = 400


def offers(req, state):
    """Public-only gate; caller already independently validated the request."""
    seat = req["seat"]
    n = len(req["plays"]) // 2 % 4
    own_played = sum(s == seat for s in req["plays"][::2])
    declaring = req["bidder"] % 2
    if (req["bid"] != 30 or seat % 2 != declaring or 7 - own_played > 3
            or n < 2 or len(state["legal"]) < 2
            or state["points"][declaring] >= 30 or state["points"][1 - declaring] > 12):
        return []
    flat = req["plays"][-2 * n:]
    trick = list(zip(flat[::2], flat[1::2]))
    partner = (seat + 2) % 4
    if winner(trick, req["decl"]) != partner:
        return []
    return [t for t in state["legal"] if sum(TILES[t]) in (5, 10)
            and winner(trick + [(seat, t)], req["decl"]) == partner]


def investigate(req, state, baseline, allowance, send):
    targets = offers(req, state)
    result = {"schema": ID, "status": "inactive", "baseline": baseline,
              "offers": targets, "choice": baseline}
    if not targets or baseline in targets:
        return baseline, result
    if allowance <= 0:
        return baseline, {**result, "status": "unresolved-no-time"}
    payload = "\n".join(k + " " + " ".join(map(str, v if isinstance(v, list) else [v]))
                        for k, v in req.items()) + "\n"
    value, status = send([str(BINARY), "--baseline", str(baseline)], payload,
                         min(MAX_SECONDS, allowance))
    if value is None:
        return baseline, {**result, "status": "unresolved", "reason": status}
    try:
        if (value.get("schema") != ID or value.get("baseline") != baseline
                or value.get("offers") != targets):
            raise ValueError("detector/identity mismatch")
        if value.get("status") == "unresolved-world-cap":
            if value.get("choice") != baseline:
                raise ValueError("refusal changed baseline")
            return baseline, value
        worlds = value.get("worlds")
        if type(worlds) is not int or not 1 <= worlds <= MAX_WORLDS:
            raise ValueError("invalid prior size")
        pairs = value.get("values")
        if (not isinstance(pairs, list) or any(not isinstance(p, list) or len(p) != 2
                or any(type(x) is not int for x in p) for p in pairs)):
            raise ValueError("malformed action comparison")
        values = dict(pairs)
        if (len(values) != len(pairs) or set(values) != {baseline, *targets}
                or any(not 0 <= x <= worlds for x in values.values())):
            raise ValueError("incomplete action comparison")
        best = baseline
        for tile in sorted(values):
            if values[tile] > values[best]:
                best = tile
        expected = "retained" if best == baseline else "changed"
        if value.get("choice") != best or value.get("status") != expected:
            raise ValueError("choice disagrees with complete comparison")
        return best, value
    except (AttributeError, TypeError, ValueError) as error:
        return baseline, {**result, "status": "unresolved-rejected", "reason": str(error)}
