"""Arbitrary precision arithmetics."""

__version__ = '0.1.0-alpha'

try:
    from ._rithm import Int
except ImportError:
    class Int(int):
        pass
