from hypothesis import given

from rithm.enums import TieBreaking
from tests.utils import pickle_round_trip
from . import strategies


@given(strategies.tie_breakings)
def test_round_trip(tie_breaking: TieBreaking) -> None:
    assert pickle_round_trip(tie_breaking) == tie_breaking
