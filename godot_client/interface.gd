extends Node

@onready var online_count = $Menu/OnlineCount
@onready var connect_button = $Login/ConnectButton
@onready var login_form = $Login
@onready var username_field = $Login/Form/UsernameField
@onready var password_field = $Login/Form/PasswordField
@onready var server_addr_field = $Login/Form/ServerAddrField
@onready var menu = $Menu
@onready var fps_counter = $FPSCounter
@onready var connection_status = $ConnectionStatus

func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("escape") and GlobalNetwork.active:
		toggle_menu()
	
	if GlobalNetwork.active:
		connection_status.text = "Connected"
	else:
		connection_status.text = "Disconnected"
	
	fps_counter.text = "FPS: " + str(Engine.get_frames_per_second())

func _on_connect_button_pressed() -> void:
	var success = GlobalNetwork.connect_to_server(server_addr_field.text)
	
	if success:
		login_form.visible = false
		menu.visible = false

func _on_disconnect_button_pressed() -> void:
	GlobalNetwork.disconnect()
	login_form.visible = true
	menu.visible = false

func toggle_menu():
	if menu.is_visible_in_tree():
		menu.visible = false
	else:
		menu.visible = true
