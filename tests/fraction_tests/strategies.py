from hypothesis import strategies

from rithm import (Int,
                   Fraction)
from tests.strategies import (decimal_int_strings_with_leading_zeros,
                              int_strings_with_bases)
from tests.utils import to_int_with_builtin_int

decimal_int_strings_with_leading_zeros = decimal_int_strings_with_leading_zeros
int_strings_with_bases = int_strings_with_bases
ints = strategies.builds(Int, decimal_int_strings_with_leading_zeros)
non_zero_ints = ints.filter(bool)
zero_ints = strategies.builds(Int)
fractions = strategies.builds(Fraction, ints, non_zero_ints)
ints_with_builtin_ints = strategies.builds(
    to_int_with_builtin_int, decimal_int_strings_with_leading_zeros)
non_zero_ints_with_non_zero_builtin_ints = ints_with_builtin_ints.filter(all)
