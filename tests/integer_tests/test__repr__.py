import sys

from hypothesis import given

from rithm.integer import Int
from . import strategies


@given(strategies.ints)
def test_round_trip(int_: Int) -> None:
    result = repr(int_)

    assert eval(result, vars(sys.modules[Int.__module__])) == int_
