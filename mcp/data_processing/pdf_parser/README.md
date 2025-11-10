# PDF Parser

A high-performance PDF text extraction tool using [extractous](https://github.com/yobix-ai/extractous).

## Features

- Fast text extraction from PDF files (25x faster than alternatives)
- Low memory footprint (11x less memory usage)
- Simple command-line interface
- Batch processing support

## Setup

### First Time Setup

1. **Download dependencies:**
   ```bash
   cd pdf_parser
   cargo build --release
   ```
   This will download the `extractous` crate and its dependencies.

### Running the Parser

1. Place your PDF files in the `data/` directory:
   - `vcf.pdf`
   - `vsphere.pdf`

2. Run the parser:
   ```bash
   cargo run --release
   ```

3. Find extracted text in `../mcp/guides/`:
   - `vcf.txt`
   - `vsphere.txt`

> **Note:** Large PDFs (100MB+) may take several minutes to process. The `--release` flag optimizes performance.

## Architecture

```
pdf_parser/
├── src/
│   └── main.rs          # Main extraction logic
├── data/
│   ├── vcf.pdf          # Input PDF (you provide)
│   └── vsphere.pdf      # Input PDF (you provide)
└── Cargo.toml

../mcp/guides/
├── vcf.txt              # Extracted output
└── vsphere.txt          # Extracted output
```

## Technical Details

This tool uses the `extractous` crate, which:
- Compiles Apache Tika as native code (no Java runtime)
- Provides excellent performance for large PDFs
- Supports various document formats beyond PDF
- Includes OCR capabilities for scanned documents

## Future Enhancements

- [ ] Markdown output with heading detection
- [ ] Configurable input/output paths via CLI args
- [ ] Progress bars for large files
- [ ] Parallel processing of multiple PDFs
