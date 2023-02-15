from hypothesis import given

from rithm.integer import Int
from tests.utils import (IntWithBuiltin,
                         equivalence)
from . import strategies


@given(strategies.ints)
def test_irreflexivity(int_: Int) -> None:
    assert not int_ != int_


@given(strategies.ints, strategies.ints)
def test_symmetry(first: Int, second: Int) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.ints, strategies.ints)
def test_equivalents(first: Int, second: Int) -> None:
    assert equivalence(first != second, not first == second)
    assert equivalence(first != second, first > second or first < second)
    assert equivalence(first != second, first > second or second > first)
    assert equivalence(first != second, second < first or second > first)
    assert equivalence(first != second, second < first or first < second)


@given(strategies.ints, strategies.ints_with_builtins)
def test_polymorphism(first: Int, second_with_builtin: IntWithBuiltin) -> None:
    second, second_builtin = second_with_builtin

    assert equivalence(first != second, first != second_builtin)


@given(strategies.ints_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(first_with_builtin: IntWithBuiltin,
                                 second_with_builtin: IntWithBuiltin) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert equivalence(first != second, first_builtin != second_builtin)
