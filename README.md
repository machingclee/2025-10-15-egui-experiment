# Video Demo

https://github.com/user-attachments/assets/0871fb9b-8043-46d9-8032-f8f79364e810

### Production Deployment

Directly run

```bash
cargo build --release
```

# Shell Script Manager

A desktop application for managing shell scripts organized in folders, built with Rust and egui.

More discussion on the project architecture will be explained in my future blog post. I have intended to manage the
application state by means of Domain Driven Design methodology.

As desktop application is a ***mix*** of frontend and backend application, I have also use "redux-like" architecture to manage the UI state. Fortunately rust has built-in mechanism for channel and messaging between threads.

## Features

- Organize scripts in hierarchical folders
- Syntax-highlighted script editing
- SQLite database for persistence
- Automatic database initialization

## Database Setup

This desktop application automatically creates and initializes the SQLite database on first run. No manual setup
required!

The database file (`database.db`) is created in the application directory. The app includes embedded migration
scripts that run automatically to set up the required tables:

For production deployment, db migration history is embedded in the binary and will be applied on first run. The `SQLite`
db
file will be saved at
`/%APP_DATE%/database.db` on windows and `~/Library/Application Support/<app-name>/database.db` on mac.

The tables include:

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

