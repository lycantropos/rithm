from hypothesis import given

from rithm import Fraction
from tests.utils import (FractionOrIntOrBuiltinInt,
                         FractionWithBuiltin,
                         IntOrBuiltin,
                         equivalence,
                         is_equivalent_to_builtin_fraction,
                         is_fraction_valid)
from . import strategies


@given(strategies.fractions)
def test_basic(fraction: Fraction) -> None:
    result = -fraction

    assert isinstance(result, Fraction)
    assert is_fraction_valid(result)


@given(strategies.fractions)
def test_fixed_point(fraction: Fraction) -> None:
    result = -fraction

    assert equivalence(fraction == result, not fraction)


@given(strategies.fractions)
def test_involution(fraction: Fraction) -> None:
    assert -(-fraction) == fraction


@given(strategies.fractions, strategies.fractions_or_ints_or_builtin_ints)
def test_add_operand(first: Fraction,
                     second: FractionOrIntOrBuiltinInt) -> None:
    assert -(first + second) == (-first) + (-second)


@given(strategies.ints_or_builtins, strategies.fractions)
def test_radd_operand(first: IntOrBuiltin, second: Fraction) -> None:
    assert -(first + second) == (-first) + (-second)


@given(strategies.fractions, strategies.fractions_or_ints_or_builtin_ints)
def test_sub_operand(first: Fraction,
                     second: FractionOrIntOrBuiltinInt) -> None:
    assert -(first - second) == (-first) - (-second)


@given(strategies.ints_or_builtins, strategies.fractions)
def test_rsub_operand(first: IntOrBuiltin, second: Fraction) -> None:
    assert -(first - second) == (-first) - (-second)


@given(strategies.fractions, strategies.fractions_or_ints_or_builtin_ints)
def test_mul_operand(first: Fraction,
                     second: FractionOrIntOrBuiltinInt) -> None:
    assert -(first * second) == (-first) * second == first * (-second)


@given(strategies.ints_or_builtins, strategies.fractions)
def test_rmul_operand(first: IntOrBuiltin, second: Fraction) -> None:
    assert -(first * second) == (-first) * second == first * (-second)


@given(strategies.fractions,
       strategies.non_zero_fractions_or_ints_or_builtin_ints)
def test_truediv_operand(first: Fraction,
                         second: FractionOrIntOrBuiltinInt) -> None:
    assert -(first / second) == (-first) / second == first / (-second)


@given(strategies.ints_or_builtins, strategies.non_zero_fractions)
def test_rtruediv_operand(first: IntOrBuiltin, second: Fraction) -> None:
    assert -(first / second) == (-first) / second == first / (-second)


@given(strategies.fractions_with_builtins)
def test_connection_with_builtin(fraction_with_builtin_fraction
                                 : FractionWithBuiltin) -> None:
    fraction, builtin_fraction = fraction_with_builtin_fraction

    assert is_equivalent_to_builtin_fraction(-fraction, -builtin_fraction)
