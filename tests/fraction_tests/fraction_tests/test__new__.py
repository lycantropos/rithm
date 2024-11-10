import fractions
from typing import Any

import pytest
from hypothesis import given

from rithm.fraction import Fraction
from tests.utils import (
    IntOrBuiltin,
    IntWithBuiltin,
    is_equivalent_to_builtin_fraction,
)

from . import strategies


def test_no_argument_connection_with_builtin() -> None:
    result = Fraction()

    assert is_equivalent_to_builtin_fraction(result, fractions.Fraction())


@given(strategies.ints_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(
    numerators: IntWithBuiltin, denominators: IntWithBuiltin
) -> None:
    numerator, builtin_numerator = numerators
    denominator, builtin_denominator = denominators

    try:
        result = Fraction(numerator, denominator)
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            fractions.Fraction(builtin_numerator, builtin_denominator)
    else:
        assert is_equivalent_to_builtin_fraction(
            result, fractions.Fraction(builtin_numerator, builtin_denominator)
        )


@given(strategies.floats)
def test_float_connection_with_builtin(float_: float) -> None:
    try:
        result = Fraction(float_)
    except (OverflowError, ValueError) as error:
        with pytest.raises(type(error)):
            fractions.Fraction(float_)
    else:
        assert is_equivalent_to_builtin_fraction(
            result, fractions.Fraction(float_)
        )


@given(strategies.builtin_fractions)
def test_rational_connection_with_builtin(
    rational: fractions.Fraction,
) -> None:
    result = Fraction(rational)

    assert is_equivalent_to_builtin_fraction(result, rational)


@given(strategies.ints_with_builtins)
def test_numerator_only_connection_with_builtin(
    numerators: IntWithBuiltin,
) -> None:
    numerator, builtin_numerator = numerators

    result = Fraction(numerator)

    assert is_equivalent_to_builtin_fraction(
        result, fractions.Fraction(builtin_numerator)
    )


@given(strategies.invalid_fractions_single_arguments)
def test_invalid_single_argument(value: Any) -> None:
    with pytest.raises(TypeError):
        Fraction(value)


@given(
    strategies.invalid_fractions_components,
    strategies.non_zero_ints_or_builtins,
)
def test_invalid_numerator(numerator: Any, denominator: IntOrBuiltin) -> None:
    with pytest.raises(TypeError):
        Fraction(numerator, denominator)


@given(strategies.ints_or_builtins, strategies.invalid_fractions_components)
def test_invalid_denominator(
    numerator: IntOrBuiltin, denominator: Any
) -> None:
    with pytest.raises(TypeError):
        Fraction(numerator, denominator)
