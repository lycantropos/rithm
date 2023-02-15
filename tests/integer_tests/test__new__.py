from typing import Tuple

import pytest
from hypothesis import given

from rithm.integer import Int
from tests.utils import is_equivalent_to_builtin_int
from . import strategies


def test_no_argument_determinism() -> None:
    result = Int()

    assert result == Int()


def test_no_argument_connection_with_builtin() -> None:
    result = Int()

    assert is_equivalent_to_builtin_int(result, int())


@given(strategies.decimal_int_strings_with_leading_zeros)
def test_decimal_string_determinism(string: str) -> None:
    result = Int(string)

    assert result == Int(string)


@given(strategies.decimal_int_strings_with_leading_zeros)
def test_decimal_string_connection_with_builtin(string: str) -> None:
    result = Int(string)

    assert is_equivalent_to_builtin_int(result, int(string))


@given(strategies.floats)
def test_float_determinism(float_: float) -> None:
    try:
        result = Int(float_)
    except (OverflowError, ValueError) as exception:
        with pytest.raises(type(exception)):
            Int(float_)
    else:
        assert result == Int(float_)


@given(strategies.floats)
def test_float_connection_with_builtin(float_: float) -> None:
    try:
        result = Int(float_)
    except (OverflowError, ValueError) as exception:
        with pytest.raises(type(exception)):
            int(float_)
    else:
        assert is_equivalent_to_builtin_int(result, int(float_))


@given(strategies.int_strings_with_bases)
def test_string_with_base_determinism(
        string_with_base: Tuple[str, int]
) -> None:
    string, base = string_with_base

    result = Int(string, base)

    assert result == Int(string, base)


@given(strategies.int_strings_with_bases)
def test_string_with_base_connection_with_builtin(
        string_with_base: Tuple[str, int]
) -> None:
    string, base = string_with_base

    result = Int(string, base)

    assert is_equivalent_to_builtin_int(result, int(string, base))


@given(strategies.int_strings, strategies.out_of_range_bases)
def test_string_with_out_of_range_base(string: int, base: int) -> None:
    with pytest.raises(ValueError):
        Int(string, base)


@given(strategies.invalid_int_strings, strategies.bases)
def test_invalid_string_with_base(string: int, base: int) -> None:
    with pytest.raises(ValueError):
        Int(string, base)
