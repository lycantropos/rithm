from __future__ import annotations

from enum import IntEnum as _IntEnum


class _BaseEnum(_IntEnum):
    __module__ = 'rithm.enums'

    def __repr__(self) -> str:
        return f'{type(self).__qualname__}.{self.name}'


class Endianness(_BaseEnum):
    BIG = 0
    LITTLE = 1


class TieBreaking(_BaseEnum):
    AWAY_FROM_ZERO = 0
    TO_EVEN = 1
    TO_ODD = 2
    TOWARD_ZERO = 3
