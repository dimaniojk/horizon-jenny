#!/usr/bin/env python3
"""Audit the real pinned export, deterministic staging and unchanged checkout."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import horizon
from test_template import audit, snapshot


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--horizon', type=Path, default=horizon.ROOT.parent/'horizon')
    args = parser.parse_args()
    checkout = args.horizon.resolve()
    status = subprocess.check_output(['git', '-C', str(checkout), 'status', '--porcelain'])
    before = {p: (checkout/p).read_bytes() for p in horizon.COMPAT['source_sha256']}
    (horizon.ROOT/'target').mkdir(exist_ok=True)
    with horizon.lock(horizon.ROOT/'target/stage.lock'):
        with tempfile.TemporaryDirectory(prefix='native-stage-test-', dir=horizon.ROOT/'target') as scratch:
            root = Path(scratch)
            output, changed = horizon.mixin_stage(checkout, root)
            assert changed
            first = snapshot(output)
            second, changed = horizon.mixin_stage(checkout, root)
            assert second == output and not changed and first == snapshot(output)
            plan = json.loads((root/'plan.json').read_text())
            assert plan['schema'] == horizon.COMPAT['mixin_schema']
            assert len(plan['injections']) == 1 and len(plan['modules']) == 1
            original = next((root/'original').glob('*/.horizon-mod-template.json')).parent
            audit(original, output, root/'plan.json')
    assert all((checkout/p).read_bytes() == content for p, content in before.items())
    assert subprocess.check_output(['git', '-C', str(checkout), 'status', '--porcelain']) == status
    print(json.dumps({'status': 'ok', 'original_checkout_unchanged': True,
                      'only_planned_asts_changed': True, 'deterministic_bytes_and_mtimes': True}, indent=2))


if __name__ == '__main__':
    main()
