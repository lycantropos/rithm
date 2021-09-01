from hypothesis import strategies

from rithm import Int
from tests.strategies import (decimal_int_strings_with_leading_zeros,
                              int_strings_with_bases)
from tests.utils import to_int_with_builtin_int

decimal_int_strings_with_leading_zeros = decimal_int_strings_with_leading_zeros
int_strings_with_bases = int_strings_with_bases
ints = strategies.builds(Int, decimal_int_strings_with_leading_zeros)
ints_with_builtin_ints = strategies.builds(
    to_int_with_builtin_int, decimal_int_strings_with_leading_zeros)
