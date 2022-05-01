from hypothesis import strategies

from rithm import (Fraction,
                   Int)
from tests.utils import (to_fraction_with_builtin,
                         to_int_with_builtin)

floats = strategies.floats()
integers = strategies.integers()
zero_integers = strategies.builds(int)
non_zero_integers = integers.filter(bool)
ints = integers.map(Int)
ints_or_builtins = ints | integers
non_zero_ints = ints.filter(bool)
zero_ints = strategies.builds(Int)
fractions = (strategies.builds(Fraction, ints)
             | strategies.builds(Fraction, ints, non_zero_ints))
zero_fractions = strategies.builds(Fraction)
non_zero_fractions = fractions.filter(bool)
fractions_or_ints_or_builtin_ints = fractions | ints | integers
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
small_ints_with_builtins = strategies.builds(to_int_with_builtin,
                                             strategies.integers(-100, 100))
