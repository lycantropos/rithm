from hypothesis import strategies

from rithm import (Fraction,
                   Int)
from tests.utils import (to_fraction_with_builtin,
                         to_int_with_builtin)

integers = strategies.integers()
ints = integers.map(Int)
non_zero_ints = ints.filter(bool)
zero_ints = strategies.builds(Int)
fractions = strategies.builds(Fraction, ints, non_zero_ints)
ints_with_builtins = strategies.builds(to_int_with_builtin, integers.map(str))
non_zero_ints_with_builtins = ints_with_builtins.filter(all)
fractions_with_builtins = strategies.builds(to_fraction_with_builtin,
                                            ints_with_builtins,
                                            non_zero_ints_with_builtins)
small_ints_with_builtins = strategies.builds(
    to_int_with_builtin, strategies.integers(-100, 100).map(str))
