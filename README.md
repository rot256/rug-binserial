# Rug-BinSerial

Rug currently serializes integers as hexadecimal strings.
This leads to a 2x overhead when using e.g. bincode.

This crate allows converting to/from a struct which is serialized as a vector of bytes.
That's it.
