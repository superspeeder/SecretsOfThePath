extends Area2D

const DUR = 8

@export var target: Node2D
@export var deactivate_after_leave: bool = true

var intensity_mapping: Dictionary[int, float]

var target_perc: float = 0.0
var perc: float = 0.0

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var setenergy = not self.overlaps_body(get_tree().get_first_node_in_group("player"));
	
	for node in target.find_children("*", "Light2D"):
		var l2: Light2D = node
		intensity_mapping[node.get_instance_id()] = l2.energy
		if setenergy:
			l2.energy = 0.0
			
	if not setenergy:
		target_perc = 1.0
		perc = 1.0

func _process(delta: float) -> void:
	if perc != target_perc:
		var dir = signf(target_perc - perc)
		perc += dir * delta * DUR
		
		if dir > 0 and target_perc < perc:
			perc = target_perc
		
		if dir < 0 and target_perc > perc:
			perc = target_perc
	
	for node in target.find_children("*", "Light2D"):
		var l2: Light2D = node
		l2.energy = intensity_mapping[l2.get_instance_id()] * perc
		
func _on_body_entered(body: Node2D) -> void:
	if body.is_in_group("player"):
		target_perc = 1.0

func _on_body_exited(body: Node2D) -> void:
	if deactivate_after_leave:
		if body.is_in_group("player"):
			target_perc = 0.0
