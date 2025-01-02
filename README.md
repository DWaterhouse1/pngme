A response to [png me](https://jrdngr.github.io/pngme_book)

This repo implements a small cli utility for emplacing, reading, and removing messages to and from .png files in a way that will be ignored by most typical png readers.

### Building

This project requires the usual `rustc` and `cargo`. Installing rust with your favourite package manager is probably sufficient.

Build simply with `cargo build`, no particular configuration required. Either run the compiled binary directly or use `cargo run`. The test suite can be run with `cargo test`.

### Usage

There are four command line options: encode, decode, remove, and print.

#### encode
`./pngme encode <PATH> <CHUNK_TYPE> <MESSAGE> [OUTPUT]`

This command adds a new chunk to the png file specified at `<PATH>`. The chunk will be saved with a chunk type given by `<CHUNK_TYPE>`. The chunk type must be four ascii alpha characters. Chunk types are used to identify the chunk's function, so it's best to avoid colliding with commonly used chunk types such as `IDAT`. The capitalisation of each letter carriers some information about the properties of the chunk, see the [png spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) for details. A reasonable choice is something like `RuSt`. The contents of the `<MESSAGE>` argument will be placed into a new chunk, appended onto the png file data. If no `[OUTPUT]` argument is specified, then the existing png file will be overwritten to add the new chunk. If an output argument is specified, the png data with the new chunk will instead be written there and the original will not be modified.

#### decode
`./pngme decode <PATH> <CHUNK_TYPE>`

This command will search the png file specified at `<PATH>` for a chunk with type specified as `<CHUNK_TYPE>`. If one is found, the data it contains will be printed. If we know a message has been written with the `encode` command to a particular chunk type, this can be used to read it. If the file contains multiple chunks with the given chunk type, this will find and print only the first one.

#### remove
`./pngme remove <PATH> <CHUNK_TYPE>`

This command will search the png file specified at `<PATH>` for a chunk with type specified as `<CHUNK_TYPE>`. If one is found, it will be removed and the contents of the png file overwritten. If the file contains multiple chunks with the given chunk type, this will find and remove only the first one.

#### print
`./pngme print <PATH>`

This command will read all the chunks contained within the png file specified at `<PATH>`, and print some information about each. Firstly, the number of total chunks is printed. Then, for each chunk in the file, its data length, chunk type, and CRC value are printed. If the data contained within the chunk is text, this will be printed also.
