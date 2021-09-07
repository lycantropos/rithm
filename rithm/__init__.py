"""Arbitrary precision arithmetic."""

__version__ = '0.2.0-alpha'

try:
    from ._rithm import Int
except ImportError:
    from math import gcd as _gcd


    class Int(int):
        def __repr__(self) -> str:
            return f'rithm.Int(\'{self}\')'

        def gcd(self, other: 'Int') -> 'Int':
            return Int(_gcd(self, other))
