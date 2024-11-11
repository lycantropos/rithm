from __future__ import annotations

from typing import TYPE_CHECKING

from typing_extensions import final as _final

if TYPE_CHECKING:
    from . import _hints

    Int = _hints.Int

    del _hints
else:
    try:
        from . import _crithm
    except ImportError:
        from ._rithm import Int
    else:
        Int = _final(_crithm.Int)

        del _crithm
