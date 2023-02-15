from hypothesis import strategies

from rithm.enums import Endianness

endianesses = strategies.sampled_from([Endianness.BIG, Endianness.LITTLE])
