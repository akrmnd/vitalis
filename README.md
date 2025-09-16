# Vitalis Studio

_Read this in [Japanese](README_ja.md)_

A desktop application for DNA/RNA sequence analysis and visualization built with Tauri and Rust.

## Features

- Parse and import FASTA/FASTQ/GenBank files
- Efficient handling of large sequences
- Sequence search and translation
- ORF (Open Reading Frame) detection
- Primer design and analysis
- Restriction enzyme site analysis
- Sequence visualization (linear and circular)
- Export to various formats (FASTA, FASTQ, GenBank, SVG, PDF)

## Development Environment Setup

### Prerequisites

- Node.js 18+
- Rust 1.70+
- npm or pnpm

### Local Development

1. Clone the repository

   ```bash
   git clone <repository-url>
   cd vitalis
   ```

2. Install dependencies

   ```bash
   npm install
   ```

3. Run the development server

   ```bash
   npm run tauri dev
   ```

### Building for Production

```bash
npm run tauri build
```

## Project Structure

```
vitalis/
├── src/                      # Frontend (React/TypeScript)
│   ├── App.tsx              # Main application component
│   ├── main.tsx             # Application entry point
│   └── styles.css           # Global styles
├── src-tauri/               # Tauri backend
│   ├── src/
│   │   └── main.rs          # Tauri application entry
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
├── vitalis-core/            # Core Rust library
│   ├── src/
│   │   ├── lib.rs           # Library root
│   │   ├── sequence.rs      # Sequence data structures
│   │   ├── feature.rs       # Feature/annotation types
│   │   ├── io/              # File I/O modules
│   │   ├── analysis/        # Analysis algorithms
│   │   └── visualization/   # Rendering modules
│   └── Cargo.toml           # Core library dependencies
├── docs/                    # Documentation
├── package.json             # Node.js dependencies
├── tsconfig.json            # TypeScript configuration
├── vite.config.ts           # Vite configuration
└── README.md                # This file
```

## Technology Stack

### Backend
- **Rust**: Core library (`vitalis-core`)
- **Tauri**: Desktop application framework
- **noodles**: Bioinformatics file formats (FASTA/FASTQ)
- **SQLite**: Local data storage

### Frontend
- **React**: UI framework
- **TypeScript**: Type safety
- **Vite**: Build tool

## Core API

### Sequence I/O
- `parse_and_import`: Import sequences from files
- `export`: Export sequences to various formats

### Sequence Operations
- `get_meta`: Get sequence metadata
- `get_window`: Retrieve sequence windows for large files
- `stats`: Calculate sequence statistics (GC%, N-ratio)

### Analysis
- `search`: Find patterns in sequences
- `translate`: Translate DNA/RNA to protein
- `find_orf`: Detect open reading frames
- `restriction_sites`: Find restriction enzyme sites

### Visualization
- `render_linear_svg`: Generate linear sequence maps
- `render_plasmid_svg`: Generate circular plasmid maps
- `export_pdf`: Export visualizations to PDF

## Performance Targets

- FASTA 100kb loading: < 400ms
- 10-mer search in 1Mbp: < 300ms
- UI scrolling: 60fps
- Undo/Redo: Instant response

## Running Tests

```bash
# Rust tests
cargo test

# Frontend tests
npm test

# Linting
cargo clippy
npm run lint

# Type checking
npm run typecheck
```

## License

This project is released under the MIT License.
