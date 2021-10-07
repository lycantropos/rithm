from hypothesis import strategies

from rithm import Int
from tests.strategies import (decimal_int_strings_with_leading_zeros,
                              int_strings_with_bases)
from tests.utils import to_int_with_builtin

decimal_int_strings_with_leading_zeros = decimal_int_strings_with_leading_zeros
int_strings_with_bases = int_strings_with_bases
ints = strategies.integers().map(Int)
ints_with_builtins = strategies.builds(to_int_with_builtin,
                                       decimal_int_strings_with_leading_zeros)
small_negative_ints_with_builtins = strategies.builds(
    to_int_with_builtin, strategies.integers(-100, -1).map(str))
small_non_negative_ints_with_builtins = strategies.builds(
    to_int_with_builtin, strategies.integers(0, 100).map(str))
