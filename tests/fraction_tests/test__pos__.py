from hypothesis import given

from rithm.fraction import Fraction
from tests.utils import is_fraction_valid

from . import strategies


@given(strategies.fractions)
def test_basic(fraction: Fraction) -> None:
    result = +fraction

    assert isinstance(result, Fraction)
    assert is_fraction_valid(result)


@given(strategies.fractions)
def test_identity(fraction: Fraction) -> None:
    assert +fraction == fraction
