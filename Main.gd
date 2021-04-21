extends Node

export(PackedScene) var mob_scene
var score

# ECS data
onready var ecs = load("res://ecs.gdns").new()
var entity_counter: int = 0

func _ready():
	randomize()

func _physics_process(delta):
	ecs.step(delta)

func game_over():
	$ScoreTimer.stop()
	$MobTimer.stop()
	$HUD.show_game_over()
#	$Music.stop()
#	$DeathSound.play()


func new_game():
	get_tree().call_group("mobs", "queue_free")
	score = 0
	$Player.start($StartPosition.position)
	$StartTimer.start()
	$HUD.update_score(score)
	$HUD.show_message("Get Ready")
#	$Music.play()


func _on_MobTimer_timeout():
	if entity_counter > 1000:
		return
	# Choose a random location on Path2D.
	var mob_spawn_location = get_node("MobPath/MobSpawnLocation")
	mob_spawn_location.unit_offset = randf()

	# Create a Mob instance and add it to the scene.
	var mob = mob_scene.instance()
	add_child(mob)

	# Set the mob's direction perpendicular to the path direction.
	var direction = mob_spawn_location.rotation + PI / 2

	# Set the mob's position to a random location.
	mob.global_position = mob_spawn_location.global_position

	# Add some randomness to the direction.
	direction += rand_range(-PI / 4, PI / 4)
	mob.rotation = direction

	# Choose the velocity.
	var velocity = Vector2(rand_range(mob.min_speed, mob.max_speed), 0).rotated(direction)

	# Add to ECS
	entity_counter += 1
	mob.entity_id = entity_counter
	ecs.register_mob(entity_counter, mob.global_position, velocity)

func _on_ScoreTimer_timeout():
	score += 1
	$HUD.update_score(score)


func _on_StartTimer_timeout():
	$MobTimer.start()
	$ScoreTimer.start()
