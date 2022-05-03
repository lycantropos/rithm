import string
from operator import itemgetter

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
invalid_int_strings = strategies.text(
        strategies.sampled_from(string.whitespace + string.punctuation)
)
int_strings = int_strings_with_bases.map(itemgetter(0))
negative_integers = strategies.integers(max_value=-1)
bases = strategies.just(0) | strategies.integers(2, 36)
out_of_range_bases = (negative_integers | strategies.just(1)
                      | strategies.sampled_from(37))
integers = strategies.integers()
non_zero_integers = integers.filter(bool)
zero_integers = strategies.builds(int)
ints = integers.map(Int)
ints_or_builtins = ints | integers
ints_with_builtins = strategies.builds(to_int_with_builtin, integers)
non_zero_ints = strategies.builds(Int, non_zero_integers)
non_zero_ints_or_builtins = non_zero_ints | non_zero_integers
zero_ints = strategies.builds(Int)
zero_ints_or_builtins = zero_ints | zero_integers
max_one_byte_signed_builtin_int = 1 << 7
negative_one_byte_integers = strategies.integers(
        -max_one_byte_signed_builtin_int, -1
)
negative_one_byte_ints = negative_one_byte_integers.map(Int)
negative_one_byte_ints_with_builtins = strategies.builds(
        to_int_with_builtin, negative_one_byte_integers
)
non_negative_one_byte_integers = strategies.integers(
        0, max_one_byte_signed_builtin_int - 1
)
non_negative_one_byte_ints = non_negative_one_byte_integers.map(Int)
non_negative_one_byte_ints_with_builtins = strategies.builds(
        to_int_with_builtin, non_negative_one_byte_integers
)
small_integers = non_negative_one_byte_integers | negative_one_byte_integers
small_ints = small_integers.map(Int)
maybe_small_integers = strategies.none() | small_integers
small_ints_with_builtins = (non_negative_one_byte_ints_with_builtins
                            | negative_one_byte_ints_with_builtins)
