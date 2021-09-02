"""Arbitrary precision arithmetic."""

__version__ = '0.1.1-alpha'

try:
    from ._rithm import Int
except ImportError:
    class Int(int):
        pass
