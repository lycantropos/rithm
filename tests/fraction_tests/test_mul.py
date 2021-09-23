from hypothesis import given

from tests.utils import (FractionWithBuiltin,
                         is_equivalent_to_builtin_fraction)
from . import strategies


@given(strategies.fractions_with_builtins,
       strategies.fractions_or_ints_with_builtins)
def test_connection_with_builtin(first_with_builtin: FractionWithBuiltin,
                                 second_with_builtin: FractionWithBuiltin
                                 ) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert is_equivalent_to_builtin_fraction(first * second,
                                             first_builtin * second_builtin)
