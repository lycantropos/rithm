from hypothesis import given

from rithm.integer import Int
from tests.utils import IntWithBuiltin, equivalence, implication

from . import strategies


@given(strategies.ints)
def test_reflexivity(int_: Int) -> None:
    assert int_ >= int_


@given(strategies.ints, strategies.ints)
def test_antisymmetry(first: Int, second: Int) -> None:
    assert equivalence(first >= second >= first, first == second)


@given(strategies.ints, strategies.ints, strategies.ints)
def test_transitivity(first: Int, second: Int, third: Int) -> None:
    assert implication(first >= second >= third, first >= third)


@given(strategies.ints, strategies.ints)
def test_alternatives(first: Int, second: Int) -> None:
    assert equivalence(first >= second, first > second or first == second)
    assert equivalence(first >= second, first > second or first == second)
    assert equivalence(first >= second, second < first or first == second)
    assert equivalence(first >= second, second < first or first == second)
    assert equivalence(first >= second, second <= first)
    assert equivalence(first >= second, not second > first)
    assert equivalence(first >= second, not first < second)


@given(strategies.ints, strategies.ints_with_builtins)
def test_polymorphism(first: Int, second_with_builtin: IntWithBuiltin) -> None:
    second, second_builtin = second_with_builtin

    assert equivalence(first >= second, first >= second_builtin)


@given(strategies.ints_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(first_with_builtin: IntWithBuiltin,
                                 second_with_builtin: IntWithBuiltin) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert equivalence(first >= second, first_builtin >= second_builtin)
