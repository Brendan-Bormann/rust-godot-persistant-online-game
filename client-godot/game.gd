extends Node

@onready var player_scene = preload("res://player.tscn")
@onready var player_container = $Players

var player_ledger: Dictionary = {}

func _process(delta: float) -> void:
	if GlobalNetwork.active:
		for id in GlobalNetwork.players:
			if player_ledger.has(id):
				var player_scene: Player = instance_from_id(player_ledger[id])
				var position = Vector3(GlobalNetwork.players[id]['position']['x'], GlobalNetwork.players[id]['position']['y'], GlobalNetwork.players[id]['position']['z'])
				player_scene.set_target_position(position)
			else:
				var player_scene = create_player(id, GlobalNetwork.players[id]['position'], GlobalNetwork.players[id]['velocity'], GlobalNetwork.players[id]['rotation'], GlobalNetwork.players[id]['username'])
				player_ledger[id] = player_scene.get_instance_id()
	else:
		for player in player_container.get_children():
			player.queue_free()
		player_ledger = {}

func create_player(id, position, velocity, rotation, player_name) -> Player:
	var new_player: Player = player_scene.instantiate()
	player_container.add_child(new_player)
	var p = new_player.spawn(int(id), Vector3(position.x, position.y, position.z), Vector3(velocity.x, velocity.y, velocity.z), rotation, player_name)
	return p

func update_player():
	pass

func remove_player():
	pass
