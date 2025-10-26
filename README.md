# Shell Script Manager

<img width="1190" height="771" alt="image" src="https://github.com/user-attachments/assets/e6ba069a-150f-44bc-acec-fe19b548c503" />

https://github.com/user-attachments/assets/693c6f3c-9678-4867-9c4d-be7f8cd04f1d



### Production Deployment

Directly run

```bash
cargo build --release
```

# Shell Script Manager

A desktop application for managing shell scripts organized in folders, built with Rust and egui.

More discussion on the project architecture can be found in [this blog post](https://machingclee.github.io/blog/article/Study-Notes-of-egui-Part-I-Architecture-of-egui-Application). I have intended to manage the
application (backend) state by means of Commands and Events for better separation of concerns.

As desktop application is a ***mix*** of frontend and backend application, I have also used "redux-like" architecture to
manage the UI state. Fortunately rust has built-in mechanism for channel and messaging between threads.

## Features

- Organize scripts in hierarchical folders
- Syntax-highlighted script editing
- SQLite database for persistence
- Automatic database initialization

## Database 

### Schema Design

<img width="708" height="497" alt="image" src="https://github.com/user-attachments/assets/bd10c18d-2eb8-4060-a6f9-79eccedc0855" />


### Setup 
This desktop application automatically creates and initializes the SQLite database on first run. No manual setup
required!

The database file (`database.db`) is created in the application directory. The app includes embedded migration
scripts that run automatically to set up the required tables.

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

