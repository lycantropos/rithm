from hypothesis import strategies

from rithm.enums import TieBreaking

tie_breakings = strategies.sampled_from([TieBreaking.AWAY_FROM_ZERO,
                                         TieBreaking.TO_EVEN,
                                         TieBreaking.TO_ODD,
                                         TieBreaking.TOWARD_ZERO])
