from typing import Tuple

from hypothesis import given

from rithm import Int
from tests.utils import is_equivalent_to_builtin_int
from . import strategies


def test_no_argument_connection_with_builtin() -> None:
    result = Int()

    assert is_equivalent_to_builtin_int(result, int())


@given(strategies.decimal_int_strings_with_leading_zeros)
def test_decimal_string_connection_with_builtin(string: str) -> None:
    result = Int(string)

    assert is_equivalent_to_builtin_int(result, int(string))


@given(strategies.int_strings_with_bases)
def test_string_with_base_connection_with_builtin(string_with_base
                                                  : Tuple[str, int]) -> None:
    string, base = string_with_base

    result = Int(string, base)

    assert is_equivalent_to_builtin_int(result, int(string, base))
