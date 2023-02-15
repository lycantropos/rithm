from hypothesis import given

from rithm.integer import Int
from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints, strategies.ints)
def test_alternatives(minuend: Int, subtrahend: Int) -> None:
    assert minuend - subtrahend == minuend + (-subtrahend)


@given(strategies.ints, strategies.ints_with_builtins)
def test_polymorphism(minuend: Int,
                      subtrahend_with_builtin: IntWithBuiltin) -> None:
    subtrahend, subtrahend_builtin = subtrahend_with_builtin

    assert minuend - subtrahend == minuend - subtrahend_builtin


@given(strategies.ints_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(
        minuend_with_builtin: IntWithBuiltin,
        subtrahend_with_builtin: IntWithBuiltin
) -> None:
    minuend, minuend_builtin = minuend_with_builtin
    subtrahend, subtrahend_builtin = subtrahend_with_builtin

    assert is_equivalent_to_builtin_int(minuend - subtrahend,
                                        minuend_builtin - subtrahend_builtin)
