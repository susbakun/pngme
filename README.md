# PNGme

A command-line tool written in Rust for encoding and decoding secret messages in PNG files using steganography. PNGme allows you to hide messages within PNG image files by embedding them as custom chunks, making them invisible to standard image viewers while preserving the image's visual appearance.

## Features

- **Encode**: Embed secret messages into PNG files using custom chunk types
- **Decode**: Extract hidden messages from PNG files
- **Remove**: Remove specific chunks from PNG files
- **Print**: Display all chunks in a PNG file for inspection

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

Extract a hidden message from a PNG file:

```bash
pngme decode <file_path> <chunk_type>
```

**Arguments:**

- `file_path`: Path to the PNG file
- `chunk_type`: The chunk type to decode

**Example:**

```bash
pngme decode image.png RuSt
```

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
3. **Validation**: Using CRC32 checksums to ensure chunk integrity

The tool maintains PNG file validity, so encoded images remain viewable in standard image viewers.

## Project Structure

```
pngme/
├── src/
│   ├── main.rs          # Entry point and CLI argument parsing
│   ├── args.rs          # Command-line argument definitions
│   ├── commands.rs      # Implementation of encode, decode, remove, print
│   ├── chunk.rs         # PNG chunk structure and operations
│   ├── chunk_type.rs    # Chunk type validation and properties
│   └── png.rs           # PNG file structure and operations
├── Cargo.toml           # Project dependencies and metadata
└── README.md            # This file
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

# 3. Decode the message
pngme decode photo_encoded.png RuSt
# Output: This is a secret message

# 4. View all chunks in the file
pngme print photo_encoded.png

# 5. Remove the secret chunk
pngme remove photo_encoded.png RuSt
```

## Error Handling

PNGme provides clear error messages for common issues:

- Invalid PNG file format
- Missing or invalid chunk types
- CRC validation failures
- File I/O errors

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

## License

[Specify your license here]

## Contributing

[Add contribution guidelines if applicable]

## Acknowledgments

This project demonstrates understanding of:

- PNG file format specification
- Binary file manipulation
- Rust's type system and error handling
- Command-line interface design
