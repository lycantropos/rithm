from __future__ import annotations

from typing import ClassVar, TYPE_CHECKING

from typing_extensions import Self as _Self, final as _final

if TYPE_CHECKING:

    @_final
    class Endianness:
        BIG: ClassVar[_Self]
        LITTLE: ClassVar[_Self]

        @property
        def value(self) -> int: ...

        def __new__(cls, value: int, /) -> _Self: ...

        def __repr__(self) -> str: ...

    @_final
    class TieBreaking:
        AWAY_FROM_ZERO: ClassVar[_Self]
        TOWARD_ZERO: ClassVar[_Self]
        TO_EVEN: ClassVar[_Self]
        TO_ODD: ClassVar[_Self]

        @property
        def value(self) -> int: ...

        def __new__(cls, value: int, /) -> _Self: ...

        def __repr__(self) -> str: ...

else:
    try:
        from . import _crithm
    except ImportError:
        from ._enums import Endianness, TieBreaking
    else:
        Endianness = _final(_crithm.Endianness)
        TieBreaking = _final(_crithm.TieBreaking)
        del _crithm
