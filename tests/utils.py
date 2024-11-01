import fractions
import pickle
import typing as t

from rithm.fraction import Fraction
from rithm.integer import Int

FractionOrIntOrBuiltinInt = t.Union[Fraction, Int, int]
FractionWithBuiltin = t.Tuple[Fraction, fractions.Fraction]
IntOrBuiltin = t.Union[Int, int]
IntWithBuiltin = t.Tuple[Int, int]
RationalWithBuiltin = t.Union[FractionWithBuiltin, IntWithBuiltin]


def equivalence(left: bool, right: bool) -> bool:
    return left is right


def implication(antecedent: bool, consequent: bool) -> bool:
    return not antecedent or consequent


def is_equivalent_to_builtin_fraction(
    value: Fraction, builtin: fractions.Fraction
) -> bool:
    return is_equivalent_to_builtin_int(
        value.numerator, builtin.numerator
    ) and is_equivalent_to_builtin_int(value.denominator, builtin.denominator)


def is_equivalent_to_builtin_int(int_: Int, builtin_int: int) -> bool:
    assert isinstance(int_, Int)
    assert isinstance(builtin_int, int)
    return int_ == builtin_int


def pickle_round_trip(value: t.Any) -> t.Any:
    return pickle.loads(pickle.dumps(value))


def to_int_with_builtin(value: int) -> IntWithBuiltin:
    return Int(value), value


def is_fraction_valid(fraction: Fraction) -> bool:
    return (
        isinstance(fraction.numerator, Int)
        and isinstance(fraction.denominator, Int)
        and fraction.denominator > 0
        and fraction.numerator.gcd(fraction.denominator) == 1
        and (fraction.numerator != 0 or fraction.denominator == 1)
    )


def to_fraction_with_builtin(
    numerators_pair: IntWithBuiltin,
    denominators_pair: t.Optional[IntWithBuiltin] = None,
) -> FractionWithBuiltin:
    numerator, builtin_numerator = numerators_pair
    if denominators_pair is None:
        return Fraction(numerator), fractions.Fraction(builtin_numerator)
    denominator, builtin_denominator = denominators_pair
    return (
        Fraction(numerator, denominator),
        fractions.Fraction(builtin_numerator, builtin_denominator),
    )
