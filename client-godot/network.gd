extends Network

@export var active = false
var poll_rate = 0.01
var timer = 0

var last_dir: Vector2
var last_rot: float
var last_pong = 0
const TIME_OUT = 5

@export var my_id = 0
@export var my_player: Node3D
@export var players: Dictionary

var my_player_last_rotation: float;

func activate(server_addr: String, name: String):
	start_server(server_addr)
	active = true
	self.send_packet(1, 0, str(name))

func deactivate():
	stop_server()
	active = false
	players = {}

func _ready() -> void:
	pass

func _physics_process(delta: float) -> void:
	if active: 
		recv_packet()
		send_dir_packet()
		send_rotation_packet()
		
		if timer > poll_rate:
			timer = 0
			self.send_packet(2, 0, "")
		else:
			timer += delta


func recv_packet():
	var result = self.read_packet()
	
	var packet_type: int = int(result[0])
	var packet_subtype: int = int(result[1])
	var payload: String = result[2]
	
	if packet_type == -1:
		return
	
	#print("GODOT: Received packet: " + result[0] + "|" + result[1])
	
	handle_packet(packet_type, packet_subtype, payload)

func handle_packet(packet_type: int, packet_subtype: int, payload: String):
	match packet_type:
		0:
			last_pong = Time.get_unix_time_from_system()
		1:
			match packet_subtype:
				0:
					var player_data = payload.split(";")
					my_id = player_data[0]
				_:
					pass
		2:
			match packet_subtype:
				0:
					handle_map_packet(payload)
		3:
			match packet_subtype:
				0:
					pass
				1:
					pass
		_:
			pass

func send_ping_packet():
	self.send_packet(0, 0, "")

func send_dir_packet():
	var dir = Input.get_vector("move_left", "move_right", "move_forward", "move_backward")
	
	if dir == last_dir:
		return
	
	self.send_packet(3, 0, str(my_id) + ";" + str(snapped(dir.x, 0.01)) + "," + str(snapped(dir.y, 0.01)))
	last_dir = dir

func send_rotation_packet():
	if self.active and self.my_player != null:
		var new_rotation = snapped(my_player.rotation.y, 0.01);
		if my_player_last_rotation != new_rotation:
			self.send_packet(3, 1, str(my_id) + ";" + str(new_rotation))
			my_player_last_rotation = new_rotation

func handle_init_packet(data):
	my_id = data['id']

func handle_map_packet(payload: String):
	var player_data = payload.split("+")
	
	for player in player_data:
		var player_values: PackedStringArray = player.split(";")
		
		if player_values.size() > 1:
			players[player_values[0]] = {
				"id": player_values[0],
				"username": player_values[1],
				"position": Vector3(float(player_values[2].split(",")[0]), float(player_values[2].split(",")[1]), float(player_values[2].split(",")[2])),
				"velocity": Vector3(float(player_values[3].split(",")[0]), float(player_values[3].split(",")[1]), float(player_values[3].split(",")[2])),
				"direction": Vector2(float(player_values[4].split(",")[0]), float(player_values[4].split(",")[1])),
				"rotation": float(player_values[5]),
				"speed": float(player_values[6]),
			}
