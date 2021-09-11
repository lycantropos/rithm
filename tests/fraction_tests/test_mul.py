import fractions
from typing import Tuple

from hypothesis import given

from rithm import Fraction
from tests.utils import is_equivalent_to_builtin_fraction
from . import strategies


@given(strategies.fractions_with_builtin_fractions,
       strategies.fractions_with_builtin_fractions)
def test_connection_with_builtin(first_with_builtin
                                 : Tuple[Fraction, fractions.Fraction],
                                 second_with_builtin
                                 : Tuple[Fraction, fractions.Fraction]
                                 ) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert is_equivalent_to_builtin_fraction(first * second,
                                             first_builtin * second_builtin)
