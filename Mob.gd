extends AnimatedSprite

#warning-ignore-all:unused_class_variable
export var min_speed = 50
export var max_speed = 100

var entity_id: int

onready var main: Node = get_parent()

onready var screen_size = get_viewport_rect().size

func _ready():
	self.playing = true
	var mob_types = self.frames.get_animation_names()
	self.animation = mob_types[randi() % mob_types.size()]

func _physics_process(_delta):
	self.global_position = main.ecs.read_data(entity_id)
	
	position.x = clamp(position.x, 0, screen_size.x)
	position.y = clamp(position.y, 0, screen_size.y)

func _on_VisibilityNotifier2D_screen_exited():
#	main.ecs.unregister_entity(entity_id)
#	queue_free()
	pass

func _on_start_game():
	main.ecs.unregister_entity(entity_id)
	queue_free()
