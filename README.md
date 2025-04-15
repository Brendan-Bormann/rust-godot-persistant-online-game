# Persistant Multiplayer Online Game
An online server-based game, using UDP. This base implementation allows users to connect and move around and see other players in the world in real time. The server runs the game and the client sends commands to be evaluated on the server.

### Building & Running

1. First, we need to build the client-rust project, which provides supporting Rust code to Godot. We can build it with `cargo build`. This will produce a `.dll` file in `/client-rust/target/debug/client_rust.dll` (unless you built with `--release`).

2. By default, the `client-godot` will see this and use it. If you want to create a client without the server for distribution, you will need to copy the above mentioned `.dll` into the `/client-godot` folder. Then you need to find `rust.gdextension` in that folder and update the file path to `res://client_rust.dll` or whatever filepath you need for the `.dll` location.

3. Last and most simply, start the rust server by going into the `/server` folder and running `cargo run` or optionally `cargo run --release`.

The server will now be running @ `0.0.0.0:8080` so find your server's IP address and share it with the clients. They will enter this address wthin the client and connect!