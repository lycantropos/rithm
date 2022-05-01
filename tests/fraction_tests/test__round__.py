from hypothesis import given

from tests.utils import (FractionWithBuiltin,
                         is_equivalent_to_builtin_fraction,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.fractions_with_builtins)
def test_no_digits_connection_with_builtin(
        fraction_with_builtin: FractionWithBuiltin
) -> None:
    fraction, builtin_fraction = fraction_with_builtin

    result = round(fraction)

    assert is_equivalent_to_builtin_int(result, round(builtin_fraction))


@given(strategies.fractions_with_builtins, strategies.small_integers)
def test_digits_connection_with_builtin(
        fraction_with_builtin: FractionWithBuiltin,
        digits: int
) -> None:
    fraction, builtin_fraction = fraction_with_builtin

    result = round(fraction, digits)

    assert is_equivalent_to_builtin_fraction(result,
                                             round(builtin_fraction, digits))
