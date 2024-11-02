extends Sprite2D


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	# Enable or disable these lights for performance purposes
	if $PointLight2D.energy == 0:
		$PointLight2D.visible = false
	else:
		$PointLight2D.visible = true
