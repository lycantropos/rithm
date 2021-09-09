from hypothesis import given

from rithm import Int
from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints, strategies.ints)
def test_alternatives(first: Int, second: Int) -> None:
    assert first - second == first + (-second)


@given(strategies.ints_with_builtin_ints, strategies.ints_with_builtin_ints)
def test_connection_with_builtin(first_with_builtin: IntWithBuiltin,
                                 second_with_builtin: IntWithBuiltin
                                 ) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert is_equivalent_to_builtin_int(first - second,
                                        first_builtin - second_builtin)
