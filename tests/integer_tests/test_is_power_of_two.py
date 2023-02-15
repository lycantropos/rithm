from hypothesis import given

from rithm import Int
from tests.utils import equivalence
from . import strategies


@given(strategies.ints)
def test_basic(int_: Int) -> None:
    assert isinstance(int_.is_power_of_two(), bool)


@given(strategies.ints)
def test_equivalents(int_: Int) -> None:
    assert equivalence(int_.is_power_of_two(),
                       int_ > 0 and not (int_ & (int_ - 1)))
