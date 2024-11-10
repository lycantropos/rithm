from hypothesis import given

from rithm.fraction import Fraction
from tests.utils import IntOrBuiltin

from . import strategies


@given(strategies.ints_or_builtins, strategies.fractions)
def test_connection_with__mul__(first: IntOrBuiltin, second: Fraction) -> None:
    assert first * second == second * first
