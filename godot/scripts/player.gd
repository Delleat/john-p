extends CharacterBody2D

@export var move_speed = 300.0
@export var jump_velocity = -400.0

@export var dash_velocity = 300.0
@export var dash_time = 8.0
@export var dash_cooldown = 5.0
@onready var dash_cooldown_o = dash_cooldown
var is_dashing = false
var dash_timer = 0.0

@onready var sprite = $AnimatedSprite

func _physics_process(delta: float) -> void:
	if Input.is_key_pressed(KEY_R):
		get_tree().reload_current_scene()
	
	if Input.is_key_pressed(KEY_ESCAPE):
		get_tree().quit()
	
	# Add the gravity.
	if not is_on_floor():
		velocity += get_gravity() * delta
	
	# Handle jump.
	if Input.is_action_just_pressed("jump") and is_on_floor():
		velocity.y = jump_velocity
		sprite.play("jump")
	
	# Get the input direction and handle the movement/deceleration.
	var direction := Input.get_axis("move_left", "move_right")
	if direction and !is_dashing:
		velocity.x = move_speed * direction
		sprite.scale.x = direction
		sprite.play("walk")
	elif !is_dashing:
		if is_on_floor(): sprite.play("idle")
		velocity.x = move_toward(velocity.x, 0, move_speed)
	
	# Dash
	if Input.is_action_just_pressed("dash") and dash_cooldown <= 0.0:
		is_dashing = true
		dash_timer = dash_time
		dash_cooldown = dash_cooldown_o
	
	if dash_timer > 0.0:
		dash_timer -= 80.0 * delta
		velocity.x += dash_velocity * sprite.scale.x
		sprite.play("dash")
	else:
		is_dashing = false
	
	if dash_cooldown >= 0.0 and !is_dashing:
		dash_cooldown -= 10.0 * delta

	move_and_slide()
