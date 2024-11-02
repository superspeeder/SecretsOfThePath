extends CharacterBody2D

@export var fixed_camera_x: bool = false
@export var fixed_camera_y: bool = false
@export var fixed_camera_marker: Marker2D

@export var trapped_in_area: bool = false
@export var trap_area: Rect2

const SPEED = 300.0

var width: float
var height: float

@export var camera: Camera2D

@export var freeze: bool = false

func _ready() -> void:
	width = $Sprite2D.get_rect().size.x
	height = $Sprite2D.get_rect().size.y

func _physics_process(delta: float) -> void:
	# Get the input direction and handle the movement/deceleration.
	# As good practice, you should replace UI actions with custom gameplay actions.
	
	if not freeze:
		var direction := Input.get_vector("ui_left", "ui_right", "ui_up", "ui_down")
		if direction.x:
			velocity.x = direction.x * SPEED
		else:
			velocity.x = move_toward(velocity.x, 0, SPEED)

		if direction.y:
			velocity.y = direction.y * SPEED
		else:
			velocity.y = move_toward(velocity.y, 0, SPEED)

		move_and_slide()
	
	if trapped_in_area:
		var min_x = trap_area.position.x
		var max_x = trap_area.position.x + trap_area.size.x - width
		var min_y = trap_area.position.y + height
		var max_y = trap_area.position.y + trap_area.size.y
		
		position.x = clampf(position.x, min_x, max_x);
		position.y = clampf(position.y, min_y, max_y);
	
	
	if fixed_camera_x:
		camera.global_position.x = fixed_camera_marker.global_position.x
	else:
		camera.global_position.x = position.x

	if fixed_camera_y:
		camera.global_position.y = fixed_camera_marker.global_position.y
	else:
		camera.global_position.y = position.y
