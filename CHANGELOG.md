# 2.0.0

There are many changes, the two formatting functions have been renamed, `format_reader_writer` now takes
a `W` and `R` instead of `&mut BufReader<W>`, it now always adds a trailing newline. `Indentation::Default` was
renamed to `Indentation::TwoSpaces` and `FourSpaces` and `Tab` were added. There may be a few more
small changes.