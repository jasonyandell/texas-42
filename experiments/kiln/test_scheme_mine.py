"""Mechanical and information-boundary checks for relational discovery."""
import copy
import re
from pathlib import Path
import tempfile
import unittest

import scheme_mine as m
from test_threat_probe import row


class RelationalChecks(unittest.TestCase):
    def test_immutable_artifact_survives_json_roundtrip_but_rejects_changes(self):
        with tempfile.TemporaryDirectory() as folder:
            path=Path(folder)/'fit.json'
            m.immutable_json(path,{'descriptor':(3,'none',('boss',))})
            m.immutable_json(path,{'descriptor':(3,'none',('boss',))})
            with self.assertRaises(ValueError):
                m.immutable_json(path,{'descriptor':(1,'none',('boss',))})

    def test_complexity_counts_predicates_not_role_declarations(self):
        self.assertEqual(m.atom_count((3,'none',('boss',))),6)
        self.assertEqual(m.atom_count((3,'beats-called',('double',))),10)

    def test_no_trump_has_no_called_boss_and_each_trump_decl_has_one(self):
        for decl in range(8):
            bosses = [tile for tile in range(28) if 'boss' in m.properties(tile,decl)]
            self.assertEqual(len(bosses),1)
            self.assertEqual(bosses[0],27 if decl == 7 else decl*(decl+1)//2+decl)
        self.assertFalse(any('boss' in m.properties(tile,9) for tile in range(28)))

    def test_every_generalization_has_its_original_world_as_witness(self):
        rows = [row(1,i,d) for i in range(2) for d in (0,5,7,9)]
        descriptors,witnesses = m.library(rows)
        lookup = {r['job_id']:r for r in rows}
        for desc in descriptors:
            self.assertTrue(m.matches(desc,lookup[witnesses[desc]['job_id']]))
            self.assertIsNone(re.search(r'\b[0-6]-[0-6]\b',m.descriptor_query(desc)))

    def test_membership_is_rotation_invariant_and_ignores_outcomes(self):
        original = row(1,0,5)
        descriptors,_ = m.library([original])
        expected = [m.matches(d,original) for d in descriptors]
        for rotated in m.mechanics_rows([original]):
            if rotated['decl'] == original['decl']:
                rotated['score'] = 42-original['score']
                self.assertEqual(expected,[m.matches(d,rotated) for d in descriptors])
        changed = copy.deepcopy(original)
        changed.update(score=0,policy_seed=9999,job_id=9876,trial=999)
        self.assertEqual(expected,[m.matches(d,changed) for d in descriptors])


if __name__ == '__main__': unittest.main()
