from __future__ import annotations

import fractions
import pickle
from typing import Any, TypeAlias

from rithm.fraction import Fraction
from rithm.integer import Int

FractionOrIntOrBuiltinInt: TypeAlias = Fraction | Int | int
FractionWithBuiltin: TypeAlias = tuple[Fraction, fractions.Fraction]
IntOrBuiltin: TypeAlias = Int | int
IntWithBuiltin: TypeAlias = tuple[Int, int]
RationalWithBuiltin: TypeAlias = FractionWithBuiltin | IntWithBuiltin


def equivalence(
    left: bool,  # noqa: FBT001
    right: bool,  # noqa: FBT001
    /,
) -> bool:
    return left is right


def implication(
    antecedent: bool,  # noqa: FBT001
    consequent: bool,  # noqa: FBT001
    /,
) -> bool:
    return not antecedent or consequent


def is_equivalent_to_builtin_fraction(
    value: Fraction, builtin: fractions.Fraction, /
) -> bool:
    return is_equivalent_to_builtin_int(
        value.numerator, builtin.numerator
    ) and is_equivalent_to_builtin_int(value.denominator, builtin.denominator)


def is_equivalent_to_builtin_int(int_: Int, builtin_int: int, /) -> bool:
    assert isinstance(int_, Int)
    assert isinstance(builtin_int, int)
    return int_ == builtin_int


def is_fraction_valid(fraction: Fraction, /) -> bool:
    return (
        isinstance(fraction.numerator, Int)
        and isinstance(fraction.denominator, Int)
        and fraction.denominator > 0
        and fraction.numerator.gcd(fraction.denominator) == 1
        and (fraction.numerator != 0 or fraction.denominator == 1)
    )


def pickling_round_trip(value: Any, /) -> Any:
    return pickle.loads(pickle.dumps(value))


def to_fraction_with_builtin(
    numerators_pair: IntWithBuiltin,
    denominators_pair: IntWithBuiltin | None = None,
    /,
) -> FractionWithBuiltin:
    numerator, builtin_numerator = numerators_pair
    if denominators_pair is None:
        return Fraction(numerator), fractions.Fraction(builtin_numerator)
    denominator, builtin_denominator = denominators_pair
    return (
        Fraction(numerator, denominator),
        fractions.Fraction(builtin_numerator, builtin_denominator),
    )


def to_int_with_builtin(value: int, /) -> IntWithBuiltin:
    return Int(value), value
