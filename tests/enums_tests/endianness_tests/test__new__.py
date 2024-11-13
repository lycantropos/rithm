import re
from typing import Any

import pytest
from hypothesis import given

from rithm.enums import Endianness
from tests.utils import equivalence

from . import strategies


@given(strategies.endiannesses_values)
def test_basic(value: int) -> None:
    result = Endianness(value)

    assert isinstance(result, Endianness)


@given(strategies.endiannesses_values, strategies.endiannesses_values)
def test_bijection(first: int, second: int) -> None:
    assert equivalence(
        first == second, Endianness(first) is Endianness(second)
    )


@given(strategies.endiannesses_values)
def test_value_round_trip(value: int) -> None:
    result = Endianness(value)

    assert result is Endianness(result.value)


@given(strategies.non_endiannesses_values)
def test_invalid_value(value: Any) -> None:
    with pytest.raises(
        ValueError,
        match=re.compile(f'{value} is not a valid {Endianness.__qualname__}'),
    ):
        Endianness(value)
