# Data Comparer

A web application built with Rust (Leptos + Axum) to compare two datasets by ID and aggregate duplicate records.

## Features

- Upload CSV, XLSX, or XLSM files
- Compare datasets by ID with automatic aggregation
- View matched records with amount differences
- See unmatched records from both datasets
- Modern web interface with real-time updates

## Use Case

Compare two datasets (e.g., Sales vs Payments) where:

- Records with the same ID are matched and compared
- Duplicate IDs within a dataset are automatically summed
- Amount differences are calculated
- Unmatched records are listed separately

### Example

**Dataset 1 (Sales):**

```
ID: 123, Name: Company A, Amount: 200
```

**Dataset 2 (Payments):**

```
ID: 123, Name: Company A, Amount: 20
ID: 123, Name: Company A, Amount: 40
```

**Result:**

```
Matched: ID 123, Sales: 200, Payments: 60 (summed), Difference: -140
```

## Technology Stack

- **Frontend**: Leptos 0.8 (Rust WebAssembly)
- **Backend**: Axum 0.8 (Async Rust web framework)
- **Parsing**: Calamine (Excel), CSV crate
- **Deployment**: Fly.io

## Project Structure

```
data-comparer/
├── backend/          # Axum API server
│   ├── src/
│   │   ├── main.rs           # Server setup
│   │   ├── handlers.rs       # Upload & compare endpoints
│   │   ├── comparison.rs     # Core comparison logic
│   │   └── parsers/          # CSV & Excel parsers
│   └── Cargo.toml
├── frontend/         # Leptos web UI
│   ├── src/
│   │   ├── main.rs           # App component
│   │   └── components/       # UI components
│   ├── index.html
│   └── Cargo.toml
├── shared/           # Common data types
│   └── src/
│       └── lib.rs            # Dataset, Record, ComparisonResult
└── Cargo.toml        # Workspace config
```

## Local Development

### Prerequisites

- Rust (edition 2024)
- Trunk (for frontend): `cargo install trunk`
- wasm32 target: `rustup target add wasm32-unknown-unknown`

### Run Backend

```bash
cd backend
cargo run
```

Backend runs on http://localhost:3000

### Run Frontend

```bash
cd frontend
trunk serve
```

Frontend runs on http://localhost:8080

### Run Tests

```bash
# All tests
cargo test --all

# Backend only
cargo test -p data-comparer-backend

# Specific test
cargo test test_aggregation
```

## Deployment

### Backend (Fly.io)

```bash
cd data-comparer
fly launch --no-deploy
fly deploy
```

Get backend URL:

```bash
fly status
# Example: https://data-comparer-backend.fly.dev
```

### Frontend (Fly.io)

1. Build with backend URL:

```bash
cd frontend
TRUNK_PUBLIC_API_URL=https://data-comparer-backend.fly.dev trunk build --release
```

2. Deploy:

```bash
fly launch --config fly.toml --no-deploy
fly deploy
```

Frontend URL: `https://data-comparer-frontend.fly.dev`

## Environment Variables

### Backend (.env or Fly secrets)

```
HOST=0.0.0.0
PORT=3000
```

### Frontend (compile-time)

```
TRUNK_PUBLIC_API_URL=https://your-backend.fly.dev
```

## API Endpoints

### POST /upload

Upload CSV or Excel file

**Request:** multipart/form-data with `file` field

**Response:**

```json
{
  "dataset": {
    "name": "Dataset",
    "records": [{ "id": "123", "name": "Company A", "amount": 1500.0 }]
  }
}
```

### POST /compare

Compare two datasets

**Request:**

```json
{
  "dataset1": {...},
  "dataset2": {...}
}
```

**Response:**

```json
{
  "result": {
    "matched": [...],
    "unmatched_from_first": [...],
    "unmatched_from_second": [...]
  }
}
```

### GET /health

Health check endpoint

## File Format

All files must have 3 columns: `id`, `name`, `amount`

### CSV Example

```csv
id,name,amount
12345,Company A,1500.00
67890,Company B,2300.50
```

### Excel Example

```
| A     | B         | C       |
|-------|-----------|---------|
| id    | name      | amount  |
| 12345 | Company A | 1500.00 |
| 67890 | Company B | 2300.50 |
```

## Comparison Logic

1. **Aggregation**: Records with duplicate IDs are summed by amount
2. **Matching**: IDs present in both datasets are matched
3. **Difference**: Amount2 - Amount1 for matched records
4. **Unmatched**: IDs only in Dataset 1 or Dataset 2

