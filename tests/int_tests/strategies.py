from hypothesis import strategies

from rithm import Int
from tests.strategies import (decimal_int_strings_with_leading_zeros,
                              int_strings_with_bases)
from tests.utils import to_int_with_builtin

floats = strategies.floats()
decimal_int_strings_with_leading_zeros = decimal_int_strings_with_leading_zeros
int_strings_with_bases = int_strings_with_bases
ints = strategies.integers().map(Int)
ints_with_builtins = strategies.builds(to_int_with_builtin,
                                       decimal_int_strings_with_leading_zeros)
max_one_byte_signed_builtin_int = 1 << 7
negative_one_byte_ints_with_builtins = strategies.builds(
        to_int_with_builtin,
        strategies.integers(-max_one_byte_signed_builtin_int, -1).map(str)
)
non_negative_one_byte_ints_with_builtins = strategies.builds(
        to_int_with_builtin,
        strategies.integers(0, max_one_byte_signed_builtin_int - 1).map(str)
)
small_ints_with_builtins = (non_negative_one_byte_ints_with_builtins
                            | negative_one_byte_ints_with_builtins)
