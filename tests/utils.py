import fractions
import pickle
from typing import (Optional,
                    Tuple,
                    TypeVar,
                    Union)

from rithm import (Fraction,
                   Int)

FractionWithBuiltin = Tuple[Fraction, fractions.Fraction]
IntOrBuiltin = Union[Int, int]
IntWithBuiltin = Tuple[Int, int]


def equivalence(left: bool, right: bool) -> bool:
    return left is right


def implication(antecedent: bool, consequent: bool) -> bool:
    return not antecedent or consequent


def is_equivalent_to_builtin_fraction(value: Fraction,
                                      builtin: fractions.Fraction) -> bool:
    return (is_equivalent_to_builtin_int(value.numerator, builtin.numerator)
            and is_equivalent_to_builtin_int(value.denominator,
                                             builtin.denominator))


def is_equivalent_to_builtin_int(value: Int, builtin: int) -> bool:
    return int(value) == builtin


_Pickleable = TypeVar('_Pickleable')


def pickle_round_trip(value: _Pickleable) -> _Pickleable:
    return pickle.loads(pickle.dumps(value))


def to_int_with_builtin(value: int) -> IntWithBuiltin:
    return Int(value), value


def to_fraction_with_builtin(numerators_pair: IntWithBuiltin,
                             denominators_pair: Optional[IntWithBuiltin] = None
                             ) -> FractionWithBuiltin:
    numerator, builtin_numerator = numerators_pair
    if denominators_pair is None:
        return Fraction(numerator), fractions.Fraction(builtin_numerator)
    denominator, builtin_denominator = denominators_pair
    return (Fraction(numerator, denominator),
            fractions.Fraction(builtin_numerator, builtin_denominator))
