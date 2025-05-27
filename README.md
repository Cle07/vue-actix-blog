# Vue + Actix Web Integration

This project demonstrates how to integrate a Vue.js frontend with an Actix Web backend in a single project.

## Project Structure

```
vue-actix/
├── Cargo.toml        # Rust backend dependencies
├── src/              # Rust backend source code
│   └── main.rs       # Actix Web server implementation
├── frontend/         # Vue.js frontend
│   ├── package.json  # Frontend dependencies
│   ├── index.html    # HTML entry point
│   ├── src/          # Vue source code
│   └── ...           # Other Vue project files
└── dev.sh            # Development workflow script
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/docs/installation)

### Development Mode (with Vue Devtools)

For the best development experience with Vue devtools:

1. Start the Actix Web backend:
   ```bash
   cargo run
   ```

2. In a separate terminal, start the Vue development server:
   ```bash
   cd frontend
   bun run dev
   ```

3. Access your application:
   - Frontend (with Vue devtools): [http://localhost:5173](http://localhost:5173)
   - Backend API: [http://localhost:8080/api](http://localhost:8080/api)

### Production Mode

1. Build the Vue frontend:
   ```bash
   cd frontend
   bun run build
   ```

2. Start the Actix Web server which will serve the Vue app:
   ```bash
   cargo run
   ```

3. Access your application at [http://localhost:8080](http://localhost:8080)

## API Endpoints

- `GET /api`: Returns a simple JSON response
- Add more API endpoints in `src/main.rs`

## Notes

- The Vue application is configured to proxy API requests to the Actix backend during development
- Static files are served from the `frontend/dist` directory in production mode
- CORS is configured to allow the Vue dev server to communicate with the Actix backend

## License

This project is licensed under the MIT License - see the LICENSE file for details.
