# Frontend

React + TypeScript + Vite frontend with shadcn/ui components.

## Development

```bash
npm install
npm run dev
```

The dev server runs on http://localhost:5173 and proxies `/api` requests to the backend at http://localhost:3000.

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run lint` - Run ESLint
- `npm run test` - Run tests with Vitest
- `npm run preview` - Preview production build

## Project Structure

```
src/
├── api/          # API client
├── components/   # React components
│   └── ui/       # shadcn/ui components
├── hooks/        # Custom React hooks
├── lib/          # Utilities
├── pages/        # Page components
├── test/         # Test setup
└── types/        # TypeScript types
    └── bindings/ # Generated from Rust (ts-rs)
```

## TypeScript Types

Types in `src/types/bindings/` are generated from Rust using ts-rs. To regenerate:

```bash
cd .. && cargo test export_bindings
```
