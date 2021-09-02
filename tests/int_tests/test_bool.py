from typing import Tuple

from hypothesis import given

from rithm import Int
from tests.utils import equivalence
from . import strategies


@given(strategies.ints_with_builtin_ints)
def test_connection_with_builtin(int_with_builtin_int: Tuple[Int, int]
                                 ) -> None:
    int_, builtin_int = int_with_builtin_int

    assert equivalence(bool(int_), bool(builtin_int))
