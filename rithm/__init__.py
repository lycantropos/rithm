"""Arbitrary precision arithmetic."""

__version__ = '9.0.0'

try:
    from ._crithm import (Endianness,
                          Fraction,
                          Int,
                          TieBreaking)
except ImportError:
    from ._rithm import (Endianness,
                         Fraction,
                         Int,
                         TieBreaking)
