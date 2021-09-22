from hypothesis import strategies

from rithm import (Fraction,
                   Int)
from tests.strategies import decimal_int_strings_with_leading_zeros
from tests.utils import (to_fraction_with_builtin_fraction,
                         to_int_with_builtin_int)

ints = strategies.integers().map(Int)
non_zero_ints = ints.filter(bool)
zero_ints = strategies.builds(Int)
fractions = strategies.builds(Fraction, ints, non_zero_ints)
ints_with_builtin_ints = strategies.builds(
    to_int_with_builtin_int, decimal_int_strings_with_leading_zeros)
non_zero_ints_with_non_zero_builtin_ints = ints_with_builtin_ints.filter(all)
fractions_with_builtin_fractions = strategies.builds(
    to_fraction_with_builtin_fraction, ints_with_builtin_ints,
    non_zero_ints_with_non_zero_builtin_ints)
small_ints_with_builtin_ints = strategies.builds(
    to_int_with_builtin_int, strategies.integers(-100, 100).map(str))
