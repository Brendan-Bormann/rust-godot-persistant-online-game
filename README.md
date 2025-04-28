# Rust / Godot MMO
A Rust-based, real-time, online, multiplayer RPG. The server uses UDP to send and receive commands and state updates with Godot clients, which use a Godot Extension written in Rust. Clients can connect to the server and they can freely move and interact with other players in real-time.

### Building & Running
1. Before starting the server, you will need to start a memory DB like Valkey, Redis, or DragonflyDB. You can use `./valkey/docker-compose.yml` to run the latest offical image of Valkey.
2. Next, we need to build the project, which provides supporting Rust code to Godot. While in the server folder (`./server`) run `cargo run` and it will start the server. In this process it'll also build the `shared` and `gd_ext` Rust libs. The `gd_ext` lib will produce a build artifact that we then need to get into the Godot Client.
3. By default, the `godot_client` will look into `/server/target/debug` for the corresponding Rust lib. If you want to export the client without the server, you will need to copy the build artifact (for example `gd_ext.dll` on Windows), and move it into the `godot_client` folder. Once in there, find `gd_ext.gdextension` and update the filepath to point to the correct artifact.

You can now start the Godot client and it should be able to connect to the server via an IP address. The server will be running @ `0.0.0.0:8080` so find your server's IP address and share it with the clients. They will enter this address wthin the client and connect!


# Gameplay
![Gameplay](https://imgur.com/a/cfaNyT6.png)

# Architecture
![Architecture](https://i.imgur.com/KOXNykq.png)
