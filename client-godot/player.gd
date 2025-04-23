class_name Player
extends Node3D

var id: int = 0
var active_player: bool = false
var my_name: String
var target_position = position
var position_lerp = 0.15
var camera_rotation_speed = 0.001

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

func set_target_position(new_position: Vector3) -> Player:
	target_position = new_position
	return self

func move():
	var prev_position = position
	position = Vector3(lerpf(prev_position.x, target_position.x, position_lerp), lerpf(prev_position.y, target_position.y, position_lerp), lerpf(prev_position.z, target_position.z, position_lerp))

func _ready() -> void:
	pass

func _process(delta: float) -> void:
	print(position)
	move()
	
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
