import pytest
from hypothesis import given

from rithm import Int
from tests.utils import (IntOrBuiltin,
                         IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints, strategies.non_zero_ints_or_builtins)
def test_basic(dividend: Int, divisor: IntOrBuiltin) -> None:
    result = divmod(dividend, divisor)

    assert isinstance(result, tuple)
    assert len(result) == 2
    assert isinstance(result[0], Int)
    assert isinstance(result[1], Int)


@given(strategies.ints, strategies.non_zero_ints_or_builtins)
def test_alternatives(dividend: Int, divisor: IntOrBuiltin) -> None:
    result = divmod(dividend, divisor)

    assert result == (dividend // divisor, dividend % divisor)


@given(strategies.ints, strategies.ints_with_builtins)
def test_polymorphism(dividend: Int, divisor_with_builtin: IntWithBuiltin
                      ) -> None:
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = divmod(dividend, divisor)
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            divmod(dividend, divisor_builtin)
    else:
        assert result == divmod(dividend, divisor_builtin)


@given(strategies.ints_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(dividend_with_builtin: IntWithBuiltin,
                                 divisor_with_builtin: IntWithBuiltin) -> None:
    dividend, dividend_builtin = dividend_with_builtin
    divisor, divisor_builtin = divisor_with_builtin

    try:
        quotient, remainder = divmod(dividend, divisor)
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            divmod(dividend_builtin, divisor_builtin)
    else:
        builtin_quotient, builtin_remainder = divmod(dividend_builtin,
                                                     divisor_builtin)

        assert is_equivalent_to_builtin_int(quotient, builtin_quotient)
        assert is_equivalent_to_builtin_int(remainder, builtin_remainder)


@given(strategies.ints, strategies.zero_ints_or_builtins)
def test_zero_divisor(dividend: Int, divisor: IntOrBuiltin) -> None:
    with pytest.raises(ZeroDivisionError):
        divmod(dividend, divisor)
