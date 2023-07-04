try:
    from ._crithm import (Endianness,
                          TieBreaking)
except ImportError:
    from ._enums import (Endianness,
                         TieBreaking)
