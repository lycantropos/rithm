"""Arbitrary precision arithmetics."""

__version__ = '0.0.0'

try:
    from ._rithm import Int
except ImportError:
    class Int(int):
        pass
