extends Control

@onready var master: HSlider = $m_settings/s_master
@onready var music: HSlider = $m_settings/s_music
@onready var sfx: HSlider = $m_settings/s_sfx

func _ready() -> void:
	load_audio_settings()

func load_audio_settings():
	AudioServer.set_bus_volume_db(0, linear_to_db(master.value))
	AudioServer.set_bus_volume_db(1, linear_to_db(music.value))
	AudioServer.set_bus_volume_db(2, linear_to_db(sfx.value))

func _on_b_play_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/main.tscn")


func _on_b_settings_pressed() -> void:
	var tween = get_tree().create_tween()
	tween.tween_property($m_main, "position", Vector2(650, $m_main.position.y), 0.2).set_trans(Tween.TRANS_SINE)
	tween.tween_property($m_settings, "position", Vector2(220.0, $m_settings.position.y), 0.2).set_trans(Tween.TRANS_SINE)


func _on_b_quit_pressed() -> void:
	get_tree().quit()


func _on_b_back_pressed() -> void:
	var tween = get_tree().create_tween()
	tween.tween_property($m_settings, "position", Vector2(-300.0, $m_settings.position.y), 0.2).set_trans(Tween.TRANS_SINE)
	tween.tween_property($m_main, "position", Vector2(260.0, $m_main.position.y), 0.2).set_trans(Tween.TRANS_SINE)

func set_volume(idx, value):
	AudioServer.set_bus_volume_db(idx, linear_to_db(value))

func _on_s_master_value_changed(value: float) -> void:
	set_volume(0, value)


func _on_s_music_value_changed(value: float) -> void:
	set_volume(1, value)


func _on_s_sfx_value_changed(value: float) -> void:
	set_volume(2, value)
