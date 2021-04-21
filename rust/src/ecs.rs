use std::collections::{vec_deque::Drain, HashMap, VecDeque};

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::RunOnce;
use gdnative::api::{Input, VisualServer};
use gdnative::prelude::*;

// TODO unused
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct Cleanup;

struct Renderable(Vector2);

// TODO unused
type Named = String;

type GodotId = i32;

struct Velocity(Vector2);

type RenderablesToRemove = Vec<String>;

struct Player;

// type Delta = f32;
#[derive(Default)]
struct Delta(f32);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum GodotInput {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

struct InputQueue {
    queue: VecDeque<GodotInput>,
}

impl InputQueue {
    pub fn new() -> Self {
        let mut queue: VecDeque<GodotInput> = VecDeque::new();
        queue.make_contiguous();
        return InputQueue { queue: queue };
    }

    pub fn add(&mut self, data: GodotInput) {
        self.queue.push_back(data);
    }

    #[warn(dead_code)]
    pub fn read_single(&mut self) -> Option<GodotInput> {
        return self.queue.pop_front();
    }

    pub fn read_all(&mut self) -> Drain<'_, GodotInput> {
        return self.queue.drain(..);
    }
}

type GodotData = HashMap<i32, Vector2>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum Stages {
    Startup,
    Preupdate,
    Update,
    Postupdate,
}

#[derive(NativeClass, Default)]
#[inherit(Reference)]
#[user_data(user_data::MutexData<ECS>)]
pub struct ECS {
    schedule: Schedule,
    world: World,
}

#[methods]
impl ECS {
    fn new(_owner: &Reference) -> Self {
        let mut ecs = ECS {
            schedule: Schedule::default(),
            world: World::default(),
        };

        // Insert resources
        ecs.world.insert_resource(RenderablesToRemove::default());
        ecs.world.insert_resource(InputQueue::new());
        ecs.world.insert_resource(GodotData::default());
        ecs.world.insert_resource(Delta::default());

        // Add stages
        ecs.schedule
            .add_stage(
                Stages::Startup,
                Schedule::default()
                    .with_run_criteria(RunOnce::default())
                    .with_stage(Stages::Startup, SystemStage::parallel()),
            )
            .add_stage(Stages::Preupdate, SystemStage::parallel())
            .add_stage(Stages::Update, SystemStage::parallel())
            .add_stage(Stages::Postupdate, SystemStage::parallel());

        // Add system
        ecs.schedule
            // Startup
            .stage(Stages::Startup, |schedule: &mut Schedule| {
                return schedule.add_system_to_stage(Stages::Startup, hello_world.system());
            })
            // Preupdate
            // .add_system_to_stage(Stages::Preupdate, cleanup_entities.system().label(Cleanup))
            .add_system_to_stage(Stages::Preupdate, handle_input.system())
            // Update
            .add_system_to_stage(Stages::Update, mob_movement.system());
        // Postupdate
        // .add_system_to_stage(Stages::Postupdate, debug_print_positions.system());

        return ecs;
    }

    #[export]
    fn step(&mut self, _owner: &Reference, delta: f32) {
        let mut delta_resource = self.world.get_resource_mut::<Delta>().unwrap();
        delta_resource.0 = delta;
        self.schedule.run(&mut self.world);
    }

    // #[export]
    // fn register_entity(&mut self, _owner: &Reference, name: String, global_position: Vector2) {
    //     self.world
    //         .spawn()
    //         .insert(Renderable::from(global_position))
    //         .insert(Named::from(&name));
    //     let mut godot_data = self.world.get_resource_mut::<GodotData>().unwrap();
    //     godot_data.insert(name, global_position);
    // }

    #[export]
    fn register_player(&mut self, _owner: &Reference, id: i32, global_position: Vector2) {
        self.world
            .spawn()
            .insert(GodotId::from(id))
            .insert(Renderable(global_position))
            .insert(Player);
        let mut godot_data = self.world.get_resource_mut::<GodotData>().unwrap();
        godot_data.insert(id, global_position);
    }

    #[export]
    fn register_mob(
        &mut self,
        _owner: &Reference,
        id: i32,
        global_position: Vector2,
        velocity: Vector2,
    ) {
        self.world
            .spawn()
            .insert(GodotId::from(id))
            .insert(Renderable(global_position))
            .insert(Velocity(velocity));
        let mut godot_data = self.world.get_resource_mut::<GodotData>().unwrap();
        godot_data.insert(id, global_position);
        // godot_print!("Rust {}: {}, {}", id, velocity.x, velocity.y)
    }

    #[export]
    fn unregister_entity(&mut self, _owner: &Reference, id: i32) {
        // let mut renderables_to_remove = self
        //     .world
        //     .get_resource_mut::<RenderablesToRemove>()
        //     .unwrap();
        // renderables_to_remove.push(name);

        let entity_to_remove = self
            .world
            .query::<(Entity, &GodotId)>()
            .iter_mut(&mut self.world)
            .find(|(_, &gid)| gid == id)
            .map(|t| t.0);

        if let Some(e) = entity_to_remove {
            self.world.despawn(e);
        }
    }

    #[export]
    fn read_input(&mut self, _owner: &Reference) {
        let mut input_queue = self.world.get_resource_mut::<InputQueue>().unwrap();
        let input_handler = Input::godot_singleton();
        if input_handler.is_action_pressed("move_up") {
            input_queue.add(GodotInput::MoveUp);
        }
        if input_handler.is_action_pressed("move_down") {
            input_queue.add(GodotInput::MoveDown);
        }
        if input_handler.is_action_pressed("move_left") {
            input_queue.add(GodotInput::MoveLeft);
        }
        if input_handler.is_action_pressed("move_right") {
            input_queue.add(GodotInput::MoveRight);
        }
    }

    #[export]
    fn read_data(&mut self, _owner: &Reference, id: i32) -> Vector2 {
        let godot_data = self.world.get_resource::<GodotData>().unwrap();

        return *godot_data.get(&id).unwrap_or(&Vector2::default());
    }
}

/*
* Systems
*/

// TODO debug
fn hello_world() {
    godot_print!("hello world");
}

// fn cleanup_entities(
//     mut commands: Commands,
//     mut renderables_to_remove: ResMut<RenderablesToRemove>,
//     mut query: Query<(Entity, &GodotId)>,
// ) {
//     for (entity, id) in query.iter_mut() {
//         if renderables_to_remove
//             .drain(..)
//             .any(|value| value == name.to_string())
//         {
//             commands.entity(entity).despawn();
//         }
//     }
// }

fn handle_input(
    mut input_queue: ResMut<InputQueue>,
    mut godot_data: ResMut<GodotData>,
    mut query: Query<(&Player, &GodotId, &mut Renderable)>,
) {
    for (p, id, mut r) in query.iter_mut() {
        for input in input_queue.read_all() {
            match input {
                GodotInput::MoveUp => r.0.y -= 1.0,
                GodotInput::MoveDown => r.0.y += 1.0,
                GodotInput::MoveLeft => r.0.x -= 1.0,
                GodotInput::MoveRight => r.0.x += 1.0,
            }
        }
        let data = godot_data.get_mut(id).unwrap();
        data.x = r.0.x;
        data.y = r.0.y;
    }
}

fn mob_movement(
    mut godot_data: ResMut<GodotData>,
    delta: Res<Delta>,
    mut query: Query<(&GodotId, &mut Renderable, &Velocity)>,
) {
    for (id, mut r, v) in query.iter_mut() {
        r.0.x += v.0.x * delta.0;
        r.0.y += v.0.y * delta.0;

        let data = godot_data.get_mut(id).unwrap();
        data.x = r.0.x;
        data.y = r.0.y;
    }
}

#[warn(dead_code)]
fn debug_print_positions(query: Query<&Renderable>) {
    for r in query.iter() {
        godot_print!("{}, {}", r.0.x, r.0.y)
    }
}
