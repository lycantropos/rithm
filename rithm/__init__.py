"""Arbitrary precision arithmetic."""

__version__ = '0.1.1'

try:
    from ._rithm import Int
except ImportError:
    from math import gcd as _gcd


    class Int(int):
        def gcd(self, other: 'Int') -> 'Int':
            return Int(_gcd(self, other))

        def __repr__(self) -> str:
            return f'rithm.Int(\'{self}\')'
