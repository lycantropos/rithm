from typing import Tuple

from hypothesis import given

from rithm import Int
from . import strategies
from ..utils import equivalence


@given(strategies.ints_with_builtin_ints, strategies.ints_with_builtin_ints)
def test_connection_with_builtin(first_int_with_builtin_int: Tuple[Int, int],
                                 second_int_with_builtin_int: Tuple[Int, int]
                                 ) -> None:
    first_int, first_builtin_int = first_int_with_builtin_int
    second_int, second_builtin_int = second_int_with_builtin_int

    assert equivalence(first_int >= second_int,
                       first_builtin_int >= second_builtin_int)
