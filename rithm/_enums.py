from __future__ import annotations

from enum import Enum as _Enum


class _BaseEnum(_Enum):
    __module__ = 'rithm.enums'

    def __repr__(self) -> str:
        return f'{type(self).__qualname__}.{self.name}'


class Endianness(_BaseEnum):
    BIG = 'big'
    LITTLE = 'little'


class TieBreaking(int, _BaseEnum):
    AWAY_FROM_ZERO = 0
    TO_EVEN = 1
    TO_ODD = 2
    TOWARD_ZERO = 3
