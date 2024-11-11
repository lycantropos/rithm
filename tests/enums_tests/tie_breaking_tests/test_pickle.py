from hypothesis import given

from rithm.enums import TieBreaking
from tests.utils import pickling_round_trip

from . import strategies


@given(strategies.tie_breakings)
def test_round_trip(tie_breaking: TieBreaking) -> None:
    assert pickling_round_trip(tie_breaking) is tie_breaking
