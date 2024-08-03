# hashfinder

Find out which part of a file a hash belongs to.

This may be useful when you reverse-engineer a file format and want to find out which part of the file is responsible for a certain hash (at least that's why I wrote this tool).

## Installation

```bash
# Build from source (requires Rust)
git clone https://github.com/Le0X8/hashfinder.git
cd hashfinder
cargo build --release

# Optional: add to PATH
sudo cp target/release/hashfinder /usr/local/bin/
```

or directly get the binary from the [releases page](https://github.com/Le0X8/hashfinder/releases) (currently only Linux & Windows amd64 binaries are provided).

## Usage

```bash
hashfinder myfile.txt crc32 ebe6c6e6
```

to find out which part of `myfile.txt` the CRC32 hash `ebe6c6e6` belongs to.

## Supported hashing algorithms

- CRC32
