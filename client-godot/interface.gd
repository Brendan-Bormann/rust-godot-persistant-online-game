extends Node

@onready var online_count = $RichTextLabel
@onready var connect_button = $ConnectButton
@onready var username_field = $UsernameField
@onready var server_addr_field = $ServerAddrField

func _process(delta: float) -> void:
	if GlobalNetwork.active == true:
		online_count.text = "Players Online: " + str(Global.players.size())
		connect_button.text = "Disconnect"
	else:
		online_count.text = "Offline"
		connect_button.text = "Connect"

func _on_connect_button_pressed() -> void:
	if !GlobalNetwork.active:
		GlobalNetwork.activate(server_addr_field.text, username_field.text)
		username_field.visible = true
		server_addr_field.visible = true
	else:
		GlobalNetwork.deactivate()
		username_field.visible = false
		server_addr_field.visible = false
