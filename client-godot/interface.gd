extends Node

@onready var online_count = $Menu/OnlineCount
@onready var connect_button = $Login/ConnectButton
@onready var login_form = $Login
@onready var username_field = $Login/Form/UsernameField
@onready var password_field = $Login/Form/PasswordField
@onready var server_addr_field = $Login/Form/ServerAddrField
@onready var menu = $Menu



func _process(delta: float) -> void:
	if GlobalNetwork.active == true:
		online_count.text = "Players Online: " + str(GlobalNetwork.players.size())
	else:
		online_count.text = ""
	
	if Input.is_action_just_pressed("escape") and GlobalNetwork.active:
		toggle_menu()

func _on_connect_button_pressed() -> void:
	if !GlobalNetwork.active:
		GlobalNetwork.activate(server_addr_field.text, username_field.text)
		login_form.visible = false

func _on_disconnect_button_pressed() -> void:
	if GlobalNetwork.active:
		GlobalNetwork.deactivate()
		login_form.visible = true
		menu.visible = false

func toggle_menu():
	if menu.is_visible_in_tree():
		menu.visible = false
	else:
		menu.visible = true
