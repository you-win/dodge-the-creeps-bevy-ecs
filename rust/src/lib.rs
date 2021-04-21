use gdnative::prelude::*;

mod ecs;

fn init(handle: InitHandle) {
    handle.add_class::<ecs::ECS>();
}

godot_init!(init);
