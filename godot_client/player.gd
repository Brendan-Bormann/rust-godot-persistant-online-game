class_name Player
extends CharacterBody3D

var id: int = 0
var active_player: bool = false
var my_name: String
var server_position = position
var server_position_lerp = 0.175
var camera_rotation_speed = 0.001
var local_speed = 600

@onready var my_cam = $CameraPivot/PlayerCamera
@onready var my_cam_pivot = $CameraPivot
@onready var my_name_tag = $NameTag

func spawn(new_id: int, new_position: Vector3, new_velocity: Vector3, new_rotation: float, new_name: String) -> Player:
	id = new_id
	position = new_position
	rotation.y = new_rotation
	my_name = new_name
	my_name_tag.text = new_name
	
	if str(id) == GlobalNetwork.my_id:
		active_player = true
		my_cam.current = true
		GlobalNetwork.my_player = self
	
	return self

func set_server_position(new_position: Vector3) -> Player:
	server_position = new_position
	return self

func lerp_towards_server_position():
	var prev_position = position
	position = Vector3(lerpf(prev_position.x, server_position.x, server_position_lerp), lerpf(prev_position.y, server_position.y, server_position_lerp), lerpf(prev_position.z, server_position.z, server_position_lerp))

func _ready() -> void:
	pass

func _physics_process(delta: float) -> void:
	lerp_towards_server_position()
	
	if active_player:
		if Input.is_action_pressed("right_mouse"):
			Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
		if Input.is_action_just_released("right_mouse"):
			Input.mouse_mode =	 Input.MOUSE_MODE_VISIBLE

func _input(event):
	if active_player:
		if event is InputEventMouseMotion and Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
			self.rotate(Vector3.UP, event.relative.x * -camera_rotation_speed)
			#GlobalNetwork.send_rot_packet(self.rotation.y)
