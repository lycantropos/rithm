from hypothesis import strategies

from rithm import Endianness

endianesses = strategies.sampled_from([Endianness.BIG, Endianness.LITTLE])
