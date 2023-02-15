try:
    from ._crithm import (Endianness,
                          TieBreaking)
except ImportError:
    from ._rithm import (Endianness,
                         TieBreaking)
