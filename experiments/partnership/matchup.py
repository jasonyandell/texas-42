"""Frozen player configurations and exact mirrored-contract scoring."""

from dataclasses import asdict, dataclass

RULES = ("fixed", "refine", "race-refine")


@dataclass(frozen=True)
class Player:
    name: str
    mode: str = "baseline"
    inner_belief: str = "voidless"
    selection: str = "fixed"
    modeled_selection: str = "fixed"
    n: int = 40
    n0: int = 8
    n1: int = 2
    budget_ms: int = 14000

    def __post_init__(self):
        if not isinstance(self.name, str) or not self.name.strip():
            raise ValueError("player needs a name")
        if self.mode not in ("phone", "baseline", "partner", "all-l1"):
            raise ValueError("unknown modeled-seat profile")
        if self.inner_belief not in ("voidless", "voids-counted"):
            raise ValueError("unknown inner belief")
        if self.selection not in RULES or self.modeled_selection not in RULES:
            raise ValueError("unknown selection rule")
        for name, low, high in [
            ("n", 1, 640),
            ("n0", 1, 64),
            ("n1", 1, 64),
            ("budget_ms", 100, 14000),
        ]:
            value = getattr(self, name)
            if type(value) is not int or not low <= value <= high:
                raise ValueError("invalid " + name)
        if self.mode == "phone" and (
            self.inner_belief != "voidless"
            or self.selection != "fixed"
            or self.modeled_selection != "fixed"
        ):
            raise ValueError(
                "phone selection/belief are fixed by the archived artifact"
            )
        if self.mode in ("baseline", "phone") and self.modeled_selection != "fixed":
            raise ValueError("no modeled L1 mind in this profile")

    def kwargs(self):
        return {k: v for k, v in asdict(self).items() if k != "name"}


def pair(a_made, b_made):
    if type(a_made) is not bool or type(b_made) is not bool:
        raise ValueError("make indicators must be Boolean")
    score = int(a_made) - int(b_made)
    return {
        "seed_delta": score,
        "wins": int(score > 0),
        "losses": int(score < 0),
        "ties": int(score == 0),
        "a_contract_wins": int(a_made) + int(not b_made),
        "b_contract_wins": int(b_made) + int(not a_made),
    }
