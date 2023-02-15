import sys

from hypothesis import given

from rithm.enums import TieBreaking
from . import strategies


@given(strategies.tie_breakings)
def test_round_trip(tie_breaking: TieBreaking) -> None:
    result = repr(tie_breaking)

    assert eval(result, sys.modules) is tie_breaking
