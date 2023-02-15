import sys

from hypothesis import given

from rithm import Int
from . import strategies


@given(strategies.ints)
def test_round_trip(int_: Int) -> None:
    result = repr(int_)

    assert eval(result, sys.modules) == int_
