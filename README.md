# img-to-ascii

A Rust command-line tool that converts images to ASCII art with multiple output styles and configurable dimensions.

## Features

- Convert any image format to ASCII art
- Multiple ASCII art styles (simple, detailed, block, intensity, minimalist, circle)
- Configurable output width
- Console or file output
- Maintains aspect ratio with character scaling adjustment

## Installation

### Prerequisites
- Rust (1.70 or later)
- Cargo

### Build from source
```bash
git clone <repository-url>
cd img-to-ascii
cargo build --release
```

## Usage

### Basic usage
```bash
cargo run -- input.jpg
```

### Advanced options
```bash
cargo run -- input.jpg --output file --name output.txt --style detailed --width 120
```

### Command-line arguments

- `input` - Path to the input image file (required)
- `--output` - Output destination: `console` (default) or `file`
- `--name` - Output filename when using file output (default: "name")
- `--style` - ASCII art style (default: "style")
- `--width` - Output width in characters (default: 80)

### ASCII Art Styles

- **simple**: ` .-+*#@`
- **detailed**: `  .':-=+o*%#@$`
- **block**: `  ░░▒▒▓▓██`
- **intensity**: ` .,:;ox%#@`
- **minimalist**: ` .-=*#`
- **circle**: ` .oO0`
- **default**: ` .:-=+*#%@`

## Examples

### Console output with detailed style
```bash
cargo run -- photo.png --style detailed --width 100
```

### Save to file with block style
```bash
cargo run -- image.jpg --output file --name ascii_art.txt --style block --width 80
```

## Dependencies

- `image` (0.24) - Image processing and format support
- `clap` (4.0) - Command-line argument parsing

## How it works

1. **Image Loading**: Uses the `image` crate to load various image formats
2. **Resizing**: Scales the image to the target width while maintaining aspect ratio
3. **Grayscale Conversion**: Converts the image to grayscale for brightness mapping
4. **ASCII Mapping**: Maps pixel brightness values to ASCII characters based on the selected style
5. **Output**: Displays the result in console or saves to a file

The tool applies a character ratio adjustment (0.45) to account for the typical height-to-width ratio of terminal characters, ensuring the ASCII art maintains proper proportions.

## Project Structure

```
src/
├── main.rs                 # CLI interface and main logic
├── lib.rs                  # Library entry point
└── services/
    ├── mod.rs              # Services module exports
    └── img_to_ascii.rs     # Core ASCII conversion logic
```

## License



🇫🇷**Usage libre, revente interdite.**\
🇬🇧**Free usage, sell it forbiden.**

### Vous pouvez / You can:

✅ Utiliser ce logiciel pour tout / Use this software for anything \
✅ Le modifier comme vous voulez / Change it however you want \
✅ Le partager avec d'autres / Share it with others \
✅ L'utiliser dans votre entreprise / Use it in your business

### Vous ne pouvez pas / You cannot:
❌ Vendre ce logiciel / Sell this software \
❌ Prétendre que c'est vous qui l'avez créé / Claim you made it

Licence complète : [LICENSE](LICENSE)\
Complete licence : [LICENSE](LICENSE)