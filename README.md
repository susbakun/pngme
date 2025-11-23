# PNGme

A command-line tool written in Rust for encoding and decoding secret messages in PNG files using steganography. PNGme allows you to hide messages within PNG image files by embedding them as custom chunks, making them invisible to standard image viewers while preserving the image's visual appearance.

## Features

- **Encode**: Embed secret messages into PNG files using custom chunk types
- **Decode**: Extract hidden messages from PNG files (with automatic detection)
- **Remove**: Remove specific chunks from PNG files
- **Print**: Display all chunks in a PNG file for inspection
- **Auto-Detection**: Automatically detect potential hidden messages without specifying chunk type
- **Library API**: Use PNGme as a library in your own Rust projects

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Build from Source

```bash
git clone <repository-url>
cd pngme
cargo build --release
```

The binary will be located at `target/release/pngme`.

## Usage

PNGme uses a subcommand-based interface. Here are the available commands:

### Encode a Message

Embed a secret message into a PNG file:

```bash
pngme encode <file_path> <chunk_type> <message> [output_file]
```

**Arguments:**

- `file_path`: Path to the input PNG file
- `chunk_type`: 4-character chunk type identifier (e.g., "RuSt", "TeSt")
- `message`: The secret message to encode
- `output_file`: (Optional) Path to save the output. If not specified, overwrites the input file

**Example:**

```bash
pngme encode image.png RuSt "Hello, World!"
pngme encode image.png TeSt "Secret message" output.png
```

### Decode a Message

Extract a hidden message from a PNG file. You can either specify the chunk type or let PNGme automatically detect it:

```bash
pngme decode <file_path> [chunk_type]
```

**Arguments:**

- `file_path`: Path to the PNG file
- `chunk_type`: (Optional) The chunk type to decode. If omitted, PNGme will automatically detect potential hidden messages

**Examples:**

```bash
# Decode a specific chunk type
pngme decode image.png RuSt

# Automatically detect and decode hidden messages
pngme decode image.png
# Output: potential hidden message: <message content>
```

**Auto-Detection:**

When no chunk type is specified, PNGme automatically searches for chunks that are not part of the standard PNG specification. It identifies potential hidden messages by comparing chunk types against a whitelist of standard chunks (IHDR, IDAT, IEND, PLTE, tRNS, etc.). If a non-standard chunk is found, it's flagged as a "potential hidden message" and its content is displayed.

### Remove a Chunk

Remove a specific chunk from a PNG file:

```bash
pngme remove <file_path> <chunk_type>
```

**Arguments:**

- `file_path`: Path to the PNG file
- `chunk_type`: The chunk type to remove

**Example:**

```bash
pngme remove image.png RuSt
```

### Print Chunks

Display all chunks in a PNG file:

```bash
pngme print <file_path>
```

**Arguments:**

- `file_path`: Path to the PNG file

**Example:**

```bash
pngme print image.png
```

## Chunk Types

Chunk types are 4-character ASCII identifiers. According to PNG specifications:

- Each character must be an ASCII letter (A-Z, a-z)
- The case of letters encodes metadata:
  - **Critical vs Ancillary**: First letter's case indicates if the chunk is critical
  - **Public vs Private**: Second letter's case indicates if the chunk is public
  - **Reserved**: Third letter's case must be uppercase for valid chunks
  - **Safe to Copy**: Fourth letter's case indicates if the chunk is safe to copy

For example, `RuSt` is a valid chunk type that is:

- Ancillary (not critical)
- Private
- Valid (reserved bit is correct)
- Safe to copy

## How It Works

PNG files are composed of chunks, each containing specific data. PNGme leverages the PNG format's extensibility by:

1. **Encoding**: Creating custom chunks with your message and inserting them into the PNG file structure
2. **Decoding**: Reading the PNG file, finding chunks by type, and extracting the message data
3. **Auto-Detection**: Scanning for non-standard chunks that may contain hidden messages
4. **Validation**: Using CRC32 checksums to ensure chunk integrity

The tool maintains PNG file validity, so encoded images remain viewable in standard image viewers.

## Using as a Library

PNGme can be used as a library in your Rust projects. Add it to your `Cargo.toml`:

```toml
[dependencies]
pngme = { path = "../pngme" }
```

Then use it in your code:

```rust
use pngme::*;

fn main() -> Result<()> {
    // Run the CLI application
    run()
}

// Or use individual functions
use pngme::{Png, Chunk, ChunkType};
use std::str::FromStr;

let mut png = Png::from_file("image.png")?;
let chunk_type = ChunkType::from_str("RuSt")?;
let chunk = Chunk::new(chunk_type, b"Hello".to_vec());
png.append_chunk(chunk);
```

## Project Structure

```
pngme/
├── src/
│   ├── lib.rs          # Library entry point and public API
│   ├── main.rs         # Binary entry point (thin CLI wrapper)
│   ├── args.rs         # Command-line argument definitions
│   ├── commands.rs     # Implementation of encode, decode, remove, print
│   ├── chunk.rs        # PNG chunk structure and operations
│   ├── chunk_type.rs   # Chunk type validation and properties
│   ├── png.rs          # PNG file structure and operations
│   └── constants.rs    # Standard PNG chunk type definitions
├── Cargo.toml          # Project dependencies and metadata
└── README.md           # This file
```

## Dependencies

- `clap`: Command-line argument parsing
- `anyhow`: Error handling
- `crc`: CRC32 checksum calculation for chunk validation

## Examples

### Complete Workflow

```bash
# 1. Encode a message into a PNG file
pngme encode photo.png RuSt "This is a secret message" photo_encoded.png

# 2. Verify the image still displays correctly (open in any image viewer)

# 3. Decode the message (with auto-detection)
pngme decode photo_encoded.png
# Output: potential hidden message: This is a secret message

# 4. Or decode a specific chunk type
pngme decode photo_encoded.png RuSt
# Output: This is a secret message

# 5. View all chunks in the file
pngme print photo_encoded.png

# 6. Remove the secret chunk
pngme remove photo_encoded.png RuSt
```

## Error Handling

PNGme provides clear error messages for common issues:

- Invalid PNG file format
- Missing or invalid chunk types
- CRC validation failures
- File I/O errors
- No hidden messages found (when using auto-detection)

## Testing

Run the test suite:

```bash
cargo test
```

The project includes comprehensive unit tests for:

- Chunk creation and validation
- Chunk type validation
- PNG file parsing
- Encoding and decoding operations
- Auto-detection functionality

## Acknowledgments

This project demonstrates understanding of:

- PNG file format specification
- Binary file manipulation
- Rust's type system and error handling
- Command-line interface design
- Library design and API structure
- Steganography techniques
