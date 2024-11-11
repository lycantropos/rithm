from hypothesis import given

from rithm.integer import Int
from tests.utils import pickling_round_trip

from . import strategies


@given(strategies.ints)
def test_round_trip(int_: Int) -> None:
    assert pickling_round_trip(int_) == int_
