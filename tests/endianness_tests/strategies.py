from hypothesis import strategies

from rithm.enums import Endianness

endiannesses = strategies.sampled_from([Endianness.BIG, Endianness.LITTLE])
