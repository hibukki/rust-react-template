# rustcard2

Production-worthy fullstack Rust + React template with:
- Real-time updates via WebSocket
- TypeScript types generated from Rust
- SQLite for no-Docker development

## Stack

**Backend**
- Axum (web framework)
- SQLx (database, SQLite)
- ts-rs (TypeScript generation)
- tokio (async runtime)
- argon2 (password hashing)

**Frontend**
- React + Vite + TypeScript
- Generated types from Rust

See [frontend/README.md](frontend/README.md)

## Quick Start

```bash
# Terminal 1: Run backend
cargo run --package api

# Terminal 2: Run frontend (dev mode with proxy)
cd frontend && npm run dev
```

The frontend proxies `/api/*` to the backend at `localhost:3000`.

## Project Structure

```
rustcard2/
├── crates/
│   ├── api/       # Axum HTTP/WS server
│   ├── shared/    # Types (with ts-rs exports)
│   ├── domain/    # Business logic (pure Rust)
│   └── db/        # Database layer (SQLx)
├── frontend/      # React Vite app
└── migrations/    # SQL migrations
```

## Generating TypeScript Types

```bash
./scripts/generate-types.sh
```

Types are exported to `frontend/src/types/bindings/`.

## API Endpoints

- `POST /api/auth/register` - Create user + profile
- `POST /api/auth/login` - Login (sets session cookie)
- `POST /api/auth/logout` - Logout
- `GET /api/profiles` - List all profiles
- `GET /api/profiles/{id}` - Get profile by ID
- `PATCH /api/profiles/{id}` - Update own profile
- `GET /api/ws` - WebSocket for real-time updates
- `GET /health` - Health check

## Real-time Updates

Connect to `/api/ws` to receive profile events:
```typescript
type WsEvent =
  | { type: "ProfileCreated", data: Profile }
  | { type: "ProfileUpdated", data: Profile }
```

## Database

Default: SQLite at `./dev.db` (auto-created on first run)

Override with `APP__DATABASE_URL`:
```bash
APP__DATABASE_URL=sqlite:./custom.db cargo run --package api
```

## Environment Variables

- `APP__PORT` - Server port (default: 3000)
- `APP__HOST` - Server host (default: 0.0.0.0)
- `APP__DATABASE_URL` - Database URL (default: sqlite:./dev.db)
