use zgrcg::Generator;

fn main() {
    Generator::builder()
        .set_output_dir("./src/generated".into())
        .set_gdextension_path("./rust.gdextension".into())
        .set_project_godot_path("./project.godot".into())
        .set_resource_path("./gd")
        .set_source_path("./src")
        .output_layer_consts()
        .output_action_consts()
        .output_action_invocations()
        .output_icon_comments() // Enable icon comment parsing, pull icons from public godot repo
        .output_scene_consts()
        .output_scene_actions()
        .add_icon_source(
            "res://icons/gd/",
            "https://raw.githubusercontent.com/godotengine/godot/refs/heads/master/editor/icons/",
        )
        .add_icon_source("res://icons/local/", "./icons")
        .generate();
}
