extends PlayerObject

@export var active: bool = false
@onready var nametag = $NameTag
@onready var cam = $CameraPivot/PlayerCamera

func _process(_delta: float) -> void:
	self.update_position()
	nametag.text = self.username
	if id == "0":
		cam.current = true
