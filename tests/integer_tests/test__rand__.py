from hypothesis import given

from rithm.integer import Int
from tests.utils import IntWithBuiltin
from . import strategies


@given(strategies.ints_with_builtins, strategies.ints)
def test_connection_with__and__(first_with_builtin: IntWithBuiltin,
                                second: Int) -> None:
    first, first_builtin = first_with_builtin

    assert first_builtin & second == first & second
