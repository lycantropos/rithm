import fractions
from typing import Tuple

from rithm import (Int,
                   Fraction)


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
