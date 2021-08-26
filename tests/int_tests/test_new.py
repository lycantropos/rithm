from typing import Tuple

from hypothesis import given

from rithm import Int
from . import strategies


@given(strategies.int_strings_with_bases)
def test_connection_with_builtin(string_with_base: Tuple[str, int]) -> None:
    string, base = string_with_base

    result = Int(string, base)

    assert str(result) == str(int(string, base))
