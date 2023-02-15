import pytest
from hypothesis import given

from rithm.fraction import Fraction
from tests.utils import (FractionWithBuiltin,
                         IntWithBuiltin,
                         is_equivalent_to_builtin_fraction)
from . import strategies


@given(strategies.fractions, strategies.small_ints_with_builtins)
def test_polymorphism(base: Fraction,
                      exponent_with_builtin: IntWithBuiltin) -> None:
    exponent, exponent_builtin = exponent_with_builtin

    try:
        result = base ** exponent
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            base ** exponent_builtin
    else:
        assert result == base ** exponent_builtin


@given(strategies.fractions_with_builtins, strategies.small_ints_with_builtins)
def test_connection_with_builtin(
        base_with_builtin: FractionWithBuiltin,
        exponent_with_builtin: IntWithBuiltin
) -> None:
    base, base_builtin = base_with_builtin
    exponent, exponent_builtin = exponent_with_builtin

    try:
        result = base ** exponent
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            base_builtin ** exponent_builtin
    else:
        assert is_equivalent_to_builtin_fraction(
                result, base_builtin ** exponent_builtin
        )
