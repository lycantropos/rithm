from hypothesis import strategies

from rithm import (Endianness,
                   Int)
from tests.strategies import (decimal_int_strings_with_leading_zeros,
                              int_strings_with_bases)
from tests.utils import to_int_with_builtin

endianesses = strategies.sampled_from([Endianness.BIG, Endianness.LITTLE])
floats = strategies.floats()
decimal_int_strings_with_leading_zeros = decimal_int_strings_with_leading_zeros
int_strings_with_bases = int_strings_with_bases
integers = strategies.integers()
ints = integers.map(Int)
ints_with_builtins = strategies.builds(to_int_with_builtin, integers)
max_one_byte_signed_builtin_int = 1 << 7
negative_one_byte_ints_with_builtins = strategies.builds(
        to_int_with_builtin,
        strategies.integers(-max_one_byte_signed_builtin_int, -1)
)
non_negative_one_byte_ints_with_builtins = strategies.builds(
        to_int_with_builtin,
        strategies.integers(0, max_one_byte_signed_builtin_int - 1)
)
small_ints_with_builtins = (non_negative_one_byte_ints_with_builtins
                            | negative_one_byte_ints_with_builtins)
