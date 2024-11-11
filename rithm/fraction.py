from __future__ import annotations

from typing import TYPE_CHECKING

from typing_extensions import final as _final

if TYPE_CHECKING:
    from . import _hints

    Fraction = _hints.Fraction

    del _hints
else:
    try:
        from . import _crithm
    except ImportError:
        from ._rithm import Fraction
    else:
        Fraction = _final(_crithm.Fraction)

        del _crithm
