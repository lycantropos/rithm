from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         pickle_round_trip)
from . import strategies


@given(strategies.ints_with_builtin_ints)
def test_round_trip(int_with_builtin_int: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin_int

    assert pickle_round_trip(int_) == int_
