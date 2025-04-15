extends Network

var active = false
var poll_rate = 0.01
var timer = 0

var last_dir: Vector2
var last_pong = 0
const TIME_OUT = 5

func activate(server_addr: String, name: String):
	start_server(server_addr)
	active = true
	self.send_packet("init", str(name))

func deactivate():
	stop_server()
	active = false
	Global.players = {}

func _ready() -> void:
	pass

func _physics_process(delta: float) -> void:
	if active: 
		recv_packet()
		send_dir_packet()
		
		if timer > poll_rate:
			timer = 0
			self.send_packet("map", "map")
		else:
			timer += delta


func recv_packet():
	var result = self.read_packet()
	var packet_type = result[0]
	var packet_data = result[1]
	
	if packet_type == "":
		return
	
	#print("GODOT: Received " + packet_type + " packet.")
	
	var json = JSON.new()
	json.parse(packet_data)
	var data = json.data
	
	if packet_type == "init" and data != null:
		handle_init_packet(data)
	if packet_type == "map" and data != null:
		handle_map_packet(data)
	if packet_type == "pong":
		last_pong = Time.get_unix_time_from_system()

func send_ping_packet():
	self.send_packet("ping", "ping")

func send_dir_packet():
	var dir = Input.get_vector("move_left", "move_right", "move_forward", "move_backward")
	
	if dir == last_dir:
		return
	
	var vector2_obj = {
		"x": dir.x,
		"y": dir.y
	}
	
	var json = JSON.new()
	var dir_data = json.stringify(vector2_obj)
	
	self.send_packet("dir", dir_data)
	last_dir = dir

func handle_init_packet(data):
	Global.my_id = data['id']
	print("set my id to ", data['id'])

func handle_map_packet(data):
	for player in data:
		Global.players[player.id] = player
