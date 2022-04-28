from hypothesis import given

from rithm import Fraction
from . import strategies


@given(strategies.fractions)
def test_idempotence(fraction_: Fraction) -> None:
    assert +fraction_ == fraction_
