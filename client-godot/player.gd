class_name Player
extends Node3D

var id: int = 0
var my_name: String
var target_position = position
var position_lerp = 0.5

@onready var my_cam = $Camera3D
@onready var my_name_tag = $NameTag

func spawn(new_id: int, new_position: Vector3, new_name: String) -> Player:
	id = new_id
	position = new_position
	my_name = new_name
	my_name_tag.text = new_name
	return self

func set_target_position(new_position: Vector3) -> Player:
	target_position = new_position
	return self

func move():
	var prev_position = position
	position = Vector3(lerpf(prev_position.x, target_position.x, position_lerp), lerpf(prev_position.y, target_position.y, position_lerp), lerpf(prev_position.z, target_position.z, position_lerp))

func _ready() -> void:
	if id == Global.my_id:
		my_cam.current = true

func _process(delta: float) -> void:
	if id == Global.my_id and !my_cam.current:
		my_cam.current = true
	
	move()
