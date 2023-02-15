from hypothesis import given

from rithm.integer import Int
from . import strategies


@given(strategies.ints)
def test_unity(int_: Int) -> None:
    assert int_.denominator == Int(1)
