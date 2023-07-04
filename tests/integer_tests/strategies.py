import string as _string
from operator import itemgetter as _itemgetter

from hypothesis import strategies as _st

from rithm.enums import Endianness as _Endianness
from rithm.integer import Int as _Int
from tests import strategies as _strategies
from tests.utils import to_int_with_builtin as _to_int_with_builtin

endianesses = _st.sampled_from([_Endianness.BIG, _Endianness.LITTLE])
floats = _st.floats()
decimal_int_strings_with_leading_zeros = (
    _strategies.decimal_int_strings_with_leading_zeros
)
int_strings_with_bases = _strategies.int_strings_with_bases
invalid_int_strings = _st.text(
        _st.sampled_from(_string.whitespace + _string.punctuation)
)
int_strings = int_strings_with_bases.map(_itemgetter(0))
negative_integers = _st.integers(max_value=-1)
bases = _st.just(0) | _st.integers(2, 36)
out_of_range_bases = negative_integers | _st.just(1) | _st.integers(37)
integers = _st.integers()
non_zero_integers = integers.filter(bool)
zero_integers = _st.builds(int)
ints = integers.map(_Int)
ints_or_builtins = ints | integers
ints_with_builtins = _st.builds(_to_int_with_builtin, integers)
non_zero_ints = _st.builds(_Int, non_zero_integers)
non_zero_ints_or_builtins = non_zero_ints | non_zero_integers
zero_ints = _st.builds(_Int)
zero_ints_or_builtins = zero_ints | zero_integers
max_one_byte_signed_builtin_int = 1 << 7
negative_one_byte_integers = _st.integers(-max_one_byte_signed_builtin_int, -1)
negative_one_byte_ints = negative_one_byte_integers.map(_Int)
negative_one_byte_ints_with_builtins = _st.builds(_to_int_with_builtin,
                                                  negative_one_byte_integers)
non_negative_one_byte_integers = _st.integers(
        0, max_one_byte_signed_builtin_int - 1
)
non_negative_one_byte_ints = non_negative_one_byte_integers.map(_Int)
non_negative_one_byte_ints_with_builtins = _st.builds(
        _to_int_with_builtin, non_negative_one_byte_integers
)
small_integers = non_negative_one_byte_integers | negative_one_byte_integers
small_ints = small_integers.map(_Int)
maybe_small_integers = _st.none() | small_integers
small_ints_with_builtins = (non_negative_one_byte_ints_with_builtins
                            | negative_one_byte_ints_with_builtins)
