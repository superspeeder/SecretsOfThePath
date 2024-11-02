extends Node2D

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	$Tiles/Floor.visible = false
	$Tiles/Walls.visible = false
	$Tiles/Walls.collision_enabled = false


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
