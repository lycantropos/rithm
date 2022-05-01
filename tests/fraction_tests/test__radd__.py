from hypothesis import given

from rithm import Fraction
from tests.utils import IntOrBuiltin
from . import strategies


@given(strategies.ints_or_builtins, strategies.fractions)
def test_connection_with_add(first: IntOrBuiltin, second: Fraction) -> None:
    assert first + second == second + first
