from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing_extensions import Self

    class Endianness:
        BIG: Endianness
        LITTLE: Endianness

        @property
        def value(self) -> int: ...

        def __new__(cls, value: int, /) -> Self: ...

        def __repr__(self) -> str: ...

    class TieBreaking:
        AWAY_FROM_ZERO: TieBreaking
        TO_EVEN: TieBreaking
        TO_ODD: TieBreaking
        TOWARD_ZERO: TieBreaking

        @property
        def value(self) -> int: ...

        def __new__(cls, value: int, /) -> Self: ...

        def __repr__(self) -> str: ...

else:
    try:
        from . import _crithm as _module
    except ImportError:
        from . import _enums as _module

    Endianness = _module.Endianness
    TieBreaking = _module.TieBreaking
