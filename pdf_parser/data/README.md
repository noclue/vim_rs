# PDF Data Directory

Place your PDF files here for processing.

## Expected Files

- `vcf.pdf` - VCF documentation
- `vsphere.pdf` - vSphere 9.0 Admin Guide

## Usage

Once PDFs are placed here, run:

```bash
cd pdf_parser
cargo run
```

The extracted text will be saved to `../mcp/guides/` as `.txt` files.
