extends Node2D

var player_inside = preload("res://parts/player.tscn")
var player_outside = preload("res://parts/player_outside.tscn")

@export var outside_keep_area: Rect2

var active_player: Node2D
var in_intro: bool = true
var pass_frame_1: bool = false

@onready var main_camera: Camera2D = %MainCamera

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var player: Node2D = player_outside.instantiate()
	player.add_to_group("player")
	player.fixed_camera_x = true
	player.fixed_camera_marker = $OutsideCameraLocation
	
	player.trapped_in_area = true
	player.trap_area = outside_keep_area
	
	main_camera.limit_bottom = $BottomLimit.position.y;
	player.camera = main_camera
	player.freeze = true
	
	main_camera.position_smoothing_speed = 0.4
	
	add_child(player)
	player.transform = $Start.transform;
	
	active_player = player


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if not pass_frame_1:
		pass_frame_1 = true
	else: # the first frame doesnt work right yet
		var distance_to_target_y = absf((main_camera.get_screen_center_position() - main_camera.get_target_position()).y)
		print(distance_to_target_y, " ", main_camera.position_smoothing_speed)
		if in_intro and distance_to_target_y < 100:
			main_camera.position_smoothing_speed = 5
		# plan: turn this into a title screen thingy
		
		if in_intro and distance_to_target_y < 2:
			active_player.freeze = false
			in_intro = false
