import fractions
from typing import Tuple

from rithm import (Fraction,
                   Int)

FractionsWithBuiltin = Tuple[Fraction, fractions.Fraction]
IntWithBuiltin = Tuple[Int, int]


def equivalence(left: bool, right: bool) -> bool:
    return left is right


def is_equivalent_to_builtin_fraction(value: Fraction,
                                      builtin: fractions.Fraction) -> bool:
    return (is_equivalent_to_builtin_int(value.numerator, builtin.numerator)
            and is_equivalent_to_builtin_int(value.denominator,
                                             builtin.denominator))


def is_equivalent_to_builtin_int(value: Int, builtin: int) -> bool:
    return str(value) == str(builtin)


def to_int_with_builtin_int(decimal_string: str) -> Tuple[Int, int]:
    return Int(decimal_string), int(decimal_string)


def to_fraction_with_builtin_fraction(numerators_pair: Tuple[Int, int],
                                      denominators_pair: Tuple[Int, int]
                                      ) -> Tuple[Fraction, fractions.Fraction]:
    numerator, builtin_numerator = numerators_pair
    denominator, builtin_denominator = denominators_pair
    return (Fraction(numerator, denominator),
            fractions.Fraction(builtin_numerator, builtin_denominator))
