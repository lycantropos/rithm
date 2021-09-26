import fractions

import pytest
from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_fraction,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints_with_builtins,
       strategies.non_negative_ints_with_builtins)
def test_non_negative_exponent_no_modulo_connection_with_builtin(
        base_with_builtin: IntWithBuiltin,
        exponent_with_builtin: IntWithBuiltin) -> None:
    base, base_builtin = base_with_builtin
    exponent, exponent_builtin = exponent_with_builtin

    assert is_equivalent_to_builtin_int(base ** exponent,
                                        base_builtin ** exponent_builtin)


@given(strategies.ints_with_builtins, strategies.negative_ints_with_builtins)
def test_negative_exponent_no_modulo_connection_with_builtin(
        base_with_builtin: IntWithBuiltin,
        exponent_with_builtin: IntWithBuiltin) -> None:
    base, base_builtin = base_with_builtin
    exponent, exponent_builtin = exponent_with_builtin

    try:
        result = base ** exponent
    except ZeroDivisionError:
        with pytest.raises(ZeroDivisionError):
            fractions.Fraction(base_builtin) ** exponent_builtin
    else:
        assert is_equivalent_to_builtin_fraction(
            result, fractions.Fraction(base_builtin) ** exponent_builtin)


@given(strategies.ints_with_builtins, strategies.ints_with_builtins,
       strategies.ints_with_builtins)
def test_with_modulo_connection_with_builtin(
        base_with_builtin: IntWithBuiltin,
        exponent_with_builtin: IntWithBuiltin,
        divisor_with_builtin: IntWithBuiltin) -> None:
    base, base_builtin = base_with_builtin
    exponent, exponent_builtin = exponent_with_builtin
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = pow(base, exponent, divisor)
    except ValueError:
        with pytest.raises(ValueError):
            pow(base_builtin, exponent_builtin, divisor_builtin)
    else:
        assert is_equivalent_to_builtin_int(
            result, pow(base_builtin, exponent_builtin, divisor_builtin))
