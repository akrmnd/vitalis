# Vitalis - Bioinformatics API

_Read this in [Japanese](README_ja.md)_

An API for analyzing and visualizing genetic sequence data.

## Features

- Parse GENBANK files
- Parse FASTA files
- Store and manage sequence data

## Development Environment Setup

### Prerequisites

- Python 3.12
- Poetry
- Docker and Docker Compose (optional)

### Local Development

1. Clone the repository

   ```bash
   git clone <repository-url>
   cd vitalis
   ```

2. Install dependencies

   ```bash
   poetry install
   ```

3. Set up environment variables

   ```bash
   cp .env.example .env
   # Edit .env file as needed
   ```

4. Run the application

   ```bash
   poetry run uvicorn src.interfaces.api.main:app --reload
   ```

5. Access API documentation
   ```
   http://localhost:8000/docs
   ```

### Running with Docker

1. Build and start the image

   ```bash
   docker-compose up -d
   ```

2. Access API documentation
   ```
   http://localhost:8000/docs
   ```

## Environment Variables

| Variable     | Description                    | Default Value             |
| ------------ | ------------------------------ | ------------------------- |
| API_HOST     | API host                       | localhost                 |
| API_PORT     | API port                       | 8000                      |
| CORS_ORIGINS | CORS origins (comma-separated) | ["http://localhost:3000"] |
| UPLOAD_DIR   | Upload directory               | uploads                   |
| OUTPUT_DIR   | Output directory               | output                    |

## API Endpoints

- `GET /`: Welcome message
- `POST /sequence/parse`: Parse sequence files
- `POST /sequence/save/genbank`: Save GENBANK records
- `POST /sequence/save/fasta`: Save FASTA records

Detailed API documentation is available at the `/docs` endpoint.

## Project Structure

```
vitalis/
├── src/                      # Source code
│   ├── application/          # Application layer
│   │   ├── dtos/             # Data Transfer Objects
│   │   └── services/         # Services
│   ├── config/               # Configuration
│   ├── domain/               # Domain layer
│   │   ├── models/           # Domain models
│   │   └── repositories/     # Repository interfaces
│   ├── infrastructure/       # Infrastructure layer
│   │   ├── parsers/          # Parsers
│   │   ├── repositories/     # Repository implementations
│   │   └── utils/            # Utilities
│   └── interfaces/           # Interface layer
│       └── api/              # API
├── tests/                    # Tests
│   └── data/                 # Test data
├── .env.example              # Environment variables example
├── .gitignore                # Git ignore settings
├── docker-compose.yml        # Docker Compose configuration
├── Dockerfile                # Docker build configuration
├── poetry.lock               # Poetry lock file
├── pyproject.toml            # Poetry project configuration
└── README.md                 # Project description
```

## Technology Stack

- **FastAPI**
- **Pydantic**
- **Poetry**
- **Docker**

## Running Tests

```bash
# Run all tests
poetry run pytest

# Run specific tests
poetry run pytest tests/test_genbank_parser.py
```

## License

This project is released under the MIT License.
