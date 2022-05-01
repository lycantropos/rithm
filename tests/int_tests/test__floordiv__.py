import math

import pytest
from hypothesis import given

from rithm import Int
from tests.utils import (IntOrBuiltin,
                         IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints, strategies.non_zero_ints_or_builtins)
def test_basic(dividend: Int, divisor: IntOrBuiltin) -> None:
    result = dividend // divisor

    assert isinstance(result, Int)


@given(strategies.ints, strategies.non_zero_ints_or_builtins)
def test_alternatives(dividend: Int, divisor: IntOrBuiltin) -> None:
    result = dividend // divisor

    assert result == math.floor((dividend - dividend % divisor) / divisor)


@given(strategies.ints, strategies.ints_with_builtins)
def test_polymorphism(dividend: Int, divisor_with_builtin: IntWithBuiltin
                      ) -> None:
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend // divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend // divisor_builtin
    else:
        assert result == dividend // divisor_builtin


@given(strategies.ints_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(dividend_with_builtin: IntWithBuiltin,
                                 divisor_with_builtin: IntWithBuiltin) -> None:
    dividend, dividend_builtin = dividend_with_builtin
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend // divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend_builtin // divisor_builtin
    else:
        assert is_equivalent_to_builtin_int(
                result, dividend_builtin // divisor_builtin
        )


@given(strategies.ints, strategies.zero_ints_or_builtins)
def test_zero_divisor(dividend: Int, divisor: IntOrBuiltin) -> None:
    with pytest.raises(ZeroDivisionError):
        dividend // divisor
