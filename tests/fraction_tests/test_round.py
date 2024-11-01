from hypothesis import given

from rithm.enums import TieBreaking
from rithm.fraction import Fraction
from rithm.integer import Int
from tests.utils import FractionWithBuiltin, is_equivalent_to_builtin_int

from . import strategies


@given(strategies.fractions, strategies.tie_breakings)
def test_basic(fraction: Fraction, tie_breaking: TieBreaking) -> None:
    result = fraction.round(tie_breaking)

    assert isinstance(result, Int)


@given(strategies.fractions, strategies.tie_breakings)
def test_value(fraction: Fraction, tie_breaking: TieBreaking) -> None:
    integer_part, fractional_part = divmod(fraction, 1)
    result = fraction.round(tie_breaking)

    assert (integer_part + (2 * fractional_part > 1)
            <= result
            <= integer_part + 1)


@given(strategies.fractions_with_builtins)
def test_connection_with__round__(
        fraction_with_builtin: FractionWithBuiltin
) -> None:
    fraction, builtin_fraction = fraction_with_builtin

    assert is_equivalent_to_builtin_int(fraction.round(TieBreaking.TO_EVEN),
                                        round(builtin_fraction))
