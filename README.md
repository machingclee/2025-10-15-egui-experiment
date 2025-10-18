# Egui study from eframe template

Study of emui and base on the template to build my own application.

## Database Setup

This application uses SQLite with Prisma for data persistence.

### Development
```bash
# Apply migrations to development database
npm run migrate:dev

# Or push schema directly (for simple schemas)
npm run db:push
```

### Production Deployment
For production, you have several options:

#### Option 1: Manual Migration (Recommended)
Run migrations as part of your deployment process:
```bash
# Apply all pending migrations
npm run migrate:deploy
```

# Shell Script Manager

A desktop application for managing shell scripts organized in folders, built with Rust and egui.

## Features

- Organize scripts in hierarchical folders
- Syntax-highlighted script editing
- SQLite database for persistence
- Automatic database initialization

## Database Setup

This desktop application automatically creates and initializes the SQLite database on first run. No manual setup required!

The database file (`dev-database.db`) is created in the application directory. The app includes embedded migration scripts that run automatically to set up the required tables:

- `scripts_folder` - Folder organization
- `shell_script` - Script storage
- `rel_scriptsfolder_shellscript` - Many-to-many relationships
- `application_state` - App settings

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

The app will automatically create the database and tables on first launch.

#### Option 3: Pre-built Database
- Create the database locally with migrations
- Include the `dev-database.db` file in your deployment package
- The app will use the existing database without running migrations

## Building
```bash
cargo build --release
```