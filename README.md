# Goaties Project

A dual-component Rust application with a frontend UI and backend server.

## Project Structure

```
Goaties/
├── Cargo.toml           # Workspace configuration
│
├── goaties-common/      # Shared types and models
│   ├── src/
│   │   ├── lib.rs       # Re-exports all modules
│   │   ├── models.rs    # FileMetadata and shared models
│   │   ├── requests.rs  # API request types
│   │   └── responses.rs # API response types
│   └── Cargo.toml
│
├── goaties-ui/          # Frontend Iced GUI application
│   ├── src/
│   │   ├── main.rs      # Iced UI application
│   │   ├── models.rs    # Re-exports from goaties-common
│   │   └── repository.rs # PoloDB document database operations
│   ├── Cargo.toml
│   └── CLAUDE.md        # Development reference for Iced 0.14 & PoloDB
│
└── goaties-backend/     # Backend REST + WebSocket server
    ├── src/
    │   ├── main.rs      # Server entry point
    │   ├── config.rs    # Configuration loader
    │   ├── rest.rs      # REST API handlers (uses goaties-common types)
    │   └── websocket.rs # WebSocket handlers
    ├── Cargo.toml
    └── config.toml      # Server configuration
```

## Backend Server

### Configuration

Edit `goaties-backend/config.toml` to configure ports and paths:

```toml
[server]
rest_port = 8080
rest_path = "/rest"

ws_port = 8081
ws_path = "/ws"

[app]
db_path = "./data/goaties.db"
```

### Running the Backend

```bash
cd goaties-backend
cargo run
```

Both servers will start concurrently:
- REST API: `http://localhost:8080/rest`
- WebSocket: `ws://localhost:8081/ws`

### REST API Endpoints

- `GET /rest/health` - Health check
- `GET /rest/files` - List all files
- `POST /rest/files` - Create file metadata

Example:
```bash
# Health check
curl http://localhost:8080/rest/health

# List files
curl http://localhost:8080/rest/files

# Create file metadata
curl -X POST http://localhost:8080/rest/files \
  -H "Content-Type: application/json" \
  -d '{"name":"test.txt","path":"/path/to/test.txt","size":1024}'
```

### WebSocket

Connect to `ws://localhost:8081/ws` for real-time communication. The server echoes back any text messages received.

Example with `websocat`:
```bash
websocat ws://localhost:8081/ws
```

## Frontend UI

### Running the UI

```bash
cd goaties-ui
cargo run
```

This launches the Iced GUI application with:
- Hello World window
- Button that shows an alert when clicked
- PoloDB document database integration for file metadata storage

### Database

The UI uses PoloDB (MongoDB-like document database) to store file metadata:
- Embedded file-based storage
- No SQL queries needed
- MongoDB-style document operations

See `goaties-ui/CLAUDE.md` for API reference.

## Shared Types (goaties-common)

The `goaties-common` crate contains shared types used by both frontend and backend:

**Models:**
- `FileMetadata` - File metadata structure with optional PoloDB support

**Requests:**
- `CreateFileRequest` - Request to create new file metadata
- `UpdateFileRequest` - Request to update file metadata
- `SearchFilesRequest` - Request to search files

**Responses:**
- `HealthResponse` - Health check response
- `FileResponse` - Single file response
- `FilesResponse` - Multiple files response
- `ErrorResponse` - Error response

The common crate uses a feature flag `polodb` to optionally include PoloDB-specific types (ObjectId) for the UI, while keeping the backend free from PoloDB dependencies.

## Development

### Technologies Used

**Backend:**
- Axum 0.8 - Web framework
- Tokio - Async runtime
- WebSocket support
- Config files (TOML)

**Frontend:**
- Iced 0.14 - Cross-platform GUI
- PoloDB 5.1 - Document database
- Serde - Serialization

### Building Both Projects

The project uses a Cargo workspace to build both frontend and backend together:

```bash
# Build everything from the root directory
cargo build

# Run specific project
cargo run -p goaties-backend
cargo run -p goaties-ui

# Or navigate to individual projects
cd goaties-backend && cargo run
cd goaties-ui && cargo run
```