# Build a CRUD API with Rust and MongoDB
Start the MongoDB server in the Docker container by running docker-compose up -d in the terminal of the root directory.
Run cargo r -r to install the necessary crates and start the Warp HTTP server.

Install the cargo-watch binary with this command:
cargo install cargo-watch 

Run the command below to build the project, start the Warp HTTP server, and reload the server when files within the src directory change.
cargo watch -q -c -w src/ -x run

