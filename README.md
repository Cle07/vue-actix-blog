# Vue + Actix Web Minimal Blog

This project demonstrates how to integrate a Vue.js frontend with an Actix Web backend in a single project.

In articles/ create a home.md and about.md then create your blog like you would make an Obsidian.

## Features

- Obsidian-Markdown parsing, including internal links, URLs, LateX, footnotes, codeblocks with highligthing.
- Timeline viewer with sorting by date.
- Luau running in backend using lua codeblocks.
- A dockerfile compatible with [fly.io](https://fly.io/).

## Project Structure

```
vue-actix/
├── Cargo.toml        # Rust backend dependencies
├── src/              # Rust backend source code
│   ├── api.rs        # Contains the API endpoints
│   ├── lua.rs        # Contains the API endpoint for Lua compilation
│   ├── db.rs         # For all interactions with sqlite
│   ├── test.rs       # Tests (UNCOMPLETE)
│   └── main.rs       # Actix Web server implementation
├── frontend/         # Vue.js frontend
│   ├── package.json  # Frontend dependencies
│   ├── index.html    # HTML entry point
│   ├── src/          # Vue source code
│   ├── public/       # For articles images
│   └── ...           # Rest of Vue project files
```

## Usage


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

You can use the Dockerfile for that, or :

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

### Timeline viewer

To create timeline articles, name your file `{YOUR_FILE_NAME}.timeline.md`. The format is :

```markdown
# 16/08/2026
LaTeX formula rendering is now available!

# 19/07/2026
Added timeline component for this changelog.
```

## License

This project is licensed under the MIT License.