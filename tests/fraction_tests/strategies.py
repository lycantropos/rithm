from hypothesis import strategies

from rithm.enums import TieBreaking
from rithm.fraction import Fraction
from rithm.integer import Int
from tests.utils import (to_fraction_with_builtin,
                         to_int_with_builtin)

tie_breakings = strategies.sampled_from([TieBreaking.AWAY_FROM_ZERO,
                                         TieBreaking.TO_EVEN,
                                         TieBreaking.TO_ODD,
                                         TieBreaking.TOWARD_ZERO])
floats = strategies.floats()
integers = strategies.integers()
zero_integers = strategies.builds(int)
non_zero_integers = integers.filter(bool)
ints = integers.map(Int)
ints_or_builtins = ints | integers
non_zero_ints = ints.filter(bool)
non_zero_ints_or_builtins = non_zero_ints | non_zero_integers
zero_ints = strategies.builds(Int)
builtin_fractions = strategies.fractions()
fractions = (strategies.builds(Fraction, ints)
             | strategies.builds(Fraction, ints, non_zero_ints))
fractions |= (strategies.builds(Fraction, ints)
              .map(lambda value: value + Fraction(1, 2)))
zero_fractions = strategies.builds(Fraction)
non_zero_fractions = fractions.filter(bool)
fractions_or_ints_or_builtin_ints = fractions | ints | integers
invalid_fractions_components = strategies.floats()
invalid_fractions_single_arguments = strategies.decimals()
ints_with_builtins = strategies.builds(to_int_with_builtin, integers)
non_zero_ints_with_builtins = ints_with_builtins.filter(all)
fractions_with_builtins = (strategies.builds(to_fraction_with_builtin,
                                             ints_with_builtins)
                           | strategies.builds(to_fraction_with_builtin,
                                               ints_with_builtins,
                                               non_zero_ints_with_builtins))
rationals_with_builtins = fractions_with_builtins | ints_with_builtins
zero_fractions_or_ints_or_builtin_ints = (zero_fractions | zero_ints
                                          | zero_integers)
non_zero_fractions_or_ints_or_builtin_ints = (
        non_zero_fractions | non_zero_ints | non_zero_integers
)
small_integers = strategies.integers(-100, 100)
small_ints_with_builtins = strategies.builds(to_int_with_builtin,
                                             small_integers)
