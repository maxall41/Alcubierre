use crate::game_object::behaviours::{EngineView, UserBehaviour};
use crate::game_object::graphics::{Graphics, GraphicsType};
use crate::game_object::physics::{PhysicsData, PhysicsObject};
use crate::physics::AlcubierreCollider;
use crate::renderer::buffer::QuadBufferBuilder;
use crate::{EngineEvent, Scene};
use hashbrown::HashSet;
use kanal::{Receiver, Sender};
use rapier2d::dynamics::{RigidBody, RigidBodyHandle};
use rapier2d::geometry::NarrowPhase;
use rapier2d::prelude::{ColliderHandle, ColliderSet, QueryPipeline, RigidBodySet, vector};
use rhai::{Engine, Scope, AST, ImmutableString, CallFnOptions};
use std::{fmt, fs};
use std::sync::Arc;
use std::time::Duration;
use rapier2d::math::Isometry;
use winit::event::VirtualKeyCode;
use crate::renderer::atlas::SpriteAtlas;
use crate::renderer::sprite::SpriteVertex;

pub mod behaviours;
pub mod graphics;
pub mod physics;

#[derive(Clone)]
pub enum GameObjectIPC {
    UserEvent(Vec<u8>), // User can use #[repr(u16)] on an enum to use this nicely
}

#[derive(Clone)]
pub struct Behaviour {
    pub ast: Arc<AST>,
    pub scope: Scope<'static>,
}

#[derive(Clone)]
pub struct GameObject {
    pub graphics: Option<GraphicsType>,
    pub behaviours: Vec<Behaviour>,
    pub pos_x: f32,
    pub pos_y: f32,
    pub physics: PhysicsData,
    pub(crate) id: u128,
    pub(crate) event_tx: Sender<GameObjectIPC>,
    pub(crate) event_rx: Receiver<GameObjectIPC>,
}

#[derive(Clone)]
pub struct GameObjectRhaiView {
    pub pos_x: f64,
    pub pos_y: f64,
    pub rigid_body: RigidBody
}

#[derive(Clone)]
pub struct EngineController {
    pub(crate) event_tx: Arc<Sender<EngineEvent>>
}

impl EngineController {
    pub fn insert_into_datamap(&mut self, var: &str, val: &str) {
        self.event_tx
            .send(EngineEvent::InsertDatamapValue((var.to_string(), val.to_string())))
            .unwrap();
    }
    pub fn set_datamap_value(&mut self, var: &str, val: &str) {
        self.event_tx
            .send(EngineEvent::SetDatamapValue((var.to_string(), val.to_string())))
            .unwrap();
    }
    pub fn remove_datamap_value(&mut self, var: &str) {
        self.event_tx
            .send(EngineEvent::RemoveDatamapValue(var.to_string()))
            .unwrap();
    }
}

#[derive(Clone)]
pub struct Input {
    pub keys_pressed: HashSet<String>
}

impl Input {
    pub fn is_key_down(&mut self, key: &str) -> bool {
        self.keys_pressed.contains(key)
    }
    pub fn is_key_up(&mut self, key: &str) -> bool {
        !self.keys_pressed.contains(key)
    }
}


impl GameObjectRhaiView {
    fn get_pos_x(&mut self) -> f64 {
        self.pos_x.clone()
    }
    fn set_pos_x(&mut self, new_val: f64) {
        self.pos_x = new_val;
    }
    fn get_pos_y(&mut self) -> f64 {
        self.pos_y.clone()
    }
    fn set_pos_y(&mut self, new_val: f64) {
        self.pos_y = new_val;
    }
    //
    fn get_rigid_body_pos_x(&mut self) -> f64 {
        let pos = self.rigid_body.position();
        pos.translation.x as f64
    }
    fn set_rigid_body_pos_x(&mut self,new_val: f64) {
        let pos = self.rigid_body.position();
        self.rigid_body.set_position(
            Isometry::new(vector![new_val as f32,pos.translation.y], 0.0),
            true,
        );
    }
    fn get_rigid_body_pos_y(&mut self) -> f64 {
        let pos = self.rigid_body.position();
        pos.translation.y as f64
    }
    fn set_rigid_body_pos_y(&mut self,new_val: f64) {
        let pos = self.rigid_body.position();
        self.rigid_body.set_position(
            Isometry::new(vector![pos.translation.x,new_val as f32], 0.0),
            true,
        );
    }
}



pub struct GameObjectView<'a> {
    pub physics: &'a mut PhysicsData,
    pub pos_x: &'a mut f32,
    pub pos_y: &'a mut f32,
}

impl GameObject {
    pub fn notify(&self, event: &[u8]) {
        let ev = event.to_vec();
        self.notify_internal(ev);
    }
    pub(crate) fn notify_internal(&self, event: Vec<u8>) {
        self.event_tx.send(GameObjectIPC::UserEvent(event));
    }
    pub(crate) fn unloading(
        &mut self,
        narrow_phase: &mut NarrowPhase,
        rigid_body_set: &mut RigidBodySet,
        tx: &mut Sender<EngineEvent>,
        keys_pressed: &mut HashSet<VirtualKeyCode>,
        key_locks: &mut HashSet<VirtualKeyCode>,
        query_pipeline: &mut QueryPipeline,
        collider_set: &mut ColliderSet,
        frame_delta: &mut Duration,
        collision_locks: &mut HashSet<ColliderHandle>,
    ) {
        // for behaviour in &mut self.behaviours {
        // behaviour.unloaded(
        //     EngineView {
        //         rigid_body_set,
        //         narrow_phase,
        //         event_tx: tx,
        //         keys_pressed,
        //         key_locks,
        //         query_pipeline,
        //         collider_set,
        //         frame_delta,
        //         collision_locks,
        //     },
        //     GameObjectView {
        //         physics: &mut self.physics,
        //         pos_x: &mut self.pos_x,
        //         pos_y: &mut self.pos_y,
        //     },
        // );
        // }
    }
    pub(crate) fn loading(
        &mut self,
        narrow_phase: &mut NarrowPhase,
        rigid_body_set: &mut RigidBodySet,
        tx: &mut Sender<EngineEvent>,
        keys_pressed: &mut HashSet<VirtualKeyCode>,
        key_locks: &mut HashSet<VirtualKeyCode>,
        query_pipeline: &mut QueryPipeline,
        collider_set: &mut ColliderSet,
        frame_delta: &mut Duration,
        collision_locks: &mut HashSet<ColliderHandle>,
    ) {
        let mut engine = Engine::new();
        for behaviour in &mut self.behaviours {
            let  options = CallFnOptions::new().rewind_scope(false);
            engine
                .call_fn_with_options::<i64>(options,&mut behaviour.scope, &behaviour.ast, "awake", ())
                .unwrap();

            //     behaviour.loaded(
        //         EngineView {
        //             rigid_body_set,
        //             narrow_phase,
        //             event_tx: tx,
        //             key_locks,
        //             keys_pressed,
        //             query_pipeline,
        //             collider_set,
        //             frame_delta,
        //             collision_locks,
        //         },
        //         GameObjectView {
        //             physics: &mut self.physics,
        //             pos_x: &mut self.pos_x,
        //             pos_y: &mut self.pos_y,
        //         },
        //     );
        }
    }
    pub(crate) fn execute(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        narrow_phase: &mut NarrowPhase,
        event_tx: &mut Sender<EngineEvent>,
        buffer: &mut QuadBufferBuilder,
        keys_pressed: &mut HashSet<VirtualKeyCode>,
        key_locks: &mut HashSet<VirtualKeyCode>,
        query_pipeline: &mut QueryPipeline,
        collider_set: &mut ColliderSet,
        frame_delta: &mut Duration,
        collision_locks: &mut HashSet<ColliderHandle>,
        sprite_verticies: &mut Vec<SpriteVertex>,
        sprite_indicies: &mut Vec<u16>,
        atlas: &SpriteAtlas
    ) {
        let event = self.event_rx.try_recv();
        let mut object_event: Option<GameObjectIPC> = None;
        match event {
            Ok(object) => {
                object_event = object;
            }
            Err(e) => {
                panic!("{}", e);
            }
        }

        let mut engine = Engine::new();
        for behaviour in &mut self.behaviours {
            let rigid_body = rigid_body_set
                .get_mut(self.physics.rigid_body_handle.unwrap())
                .unwrap();

            let mut pt = GameObjectRhaiView {
                pos_x: self.pos_x as f64,
                pos_y: self.pos_y as f64,
                rigid_body:rigid_body.clone(),
            };

            let mut keys_pressed_simplified: HashSet<String> = HashSet::new();

            for key in keys_pressed.iter() {
                keys_pressed_simplified.insert(format!("{:?}", key).to_ascii_lowercase());
            }

            let rh_input = Input {
                keys_pressed: keys_pressed_simplified
            };

            let engine_controller = EngineController {
                event_tx: Arc::new(event_tx.clone())
            };

            behaviour.scope.set_or_push("self", pt.clone());
            behaviour.scope.set_or_push("Input", rh_input);
            behaviour.scope.set_or_push("engine", engine_controller);


            engine
                .register_type::<GameObjectRhaiView>()
                .register_get_set(
                    "pos_x",
                    GameObjectRhaiView::get_pos_x,
                    GameObjectRhaiView::set_pos_x,
                )
                .register_get_set(
                    "pos_y",
                    GameObjectRhaiView::get_pos_y,
                    GameObjectRhaiView::set_pos_y,
                )
                .register_get_set(
                    "rigid_body_pos_x",
                    GameObjectRhaiView::get_rigid_body_pos_x,
                    GameObjectRhaiView::set_rigid_body_pos_x,
                )
                .register_get_set(
                "rigid_body_pos_y",
                GameObjectRhaiView::get_rigid_body_pos_y,
                GameObjectRhaiView::set_rigid_body_pos_y,
                );

            engine.register_type::<Input>()
                .register_fn("is_key_down", Input::is_key_down)
                .register_fn("is_key_up", Input::is_key_up);

            engine.register_type::<EngineController>()
                .register_fn("insert_into_datamap", EngineController::insert_into_datamap)
                .register_fn("set_datamap_value", EngineController::set_datamap_value)
                .register_fn("remove_datamap_value", EngineController::remove_datamap_value);
            let mut options = CallFnOptions::new().rewind_scope(false);
            let new_view = engine
                .call_fn_with_options::<GameObjectRhaiView>(options,&mut behaviour.scope, &behaviour.ast, "update", (frame_delta.as_secs_f64(),))
                .unwrap();

            //println!("OLD: {}", self.pos_x);
            //println!("NEW: {}", new_view.pos_x);


            rigid_body.set_position(new_view.rigid_body.position().clone(),true)
            //TODO: Maybe this should be a drain so all events get sent per frame
            // if object_event.is_some() {
            //     behaviour.received_event(
            //         object_event.as_ref().unwrap(),
            //         EngineView {
            //             rigid_body_set,
            //             narrow_phase,
            //             event_tx,
            //             keys_pressed,
            //             key_locks,
            //             query_pipeline,
            //             collider_set,
            //             frame_delta,
            //             collision_locks,
            //         },
            //         GameObjectView {
            //             physics: &mut self.physics,
            //             pos_x: &mut self.pos_x,
            //             pos_y: &mut self.pos_y,
            //         },
            //     );
            // }
            // behaviour.game_loop(
            //     GameObjectView {
            //         physics: &mut self.physics,
            //         pos_x: &mut self.pos_x,
            //         pos_y: &mut self.pos_y,
            //     },
            //     EngineView {
            //         rigid_body_set,
            //         narrow_phase,
            //         event_tx,
            //         keys_pressed,
            //         key_locks,
            //         query_pipeline,
            //         collider_set,
            //         frame_delta,
            //         collision_locks,
            //     },
            // );
        }

        if self.physics.rigid_body_handle.is_some() {
            let new_pos = self.get_updated_physics_position(rigid_body_set);
            self.pos_x = new_pos.0;
            self.pos_y = new_pos.1;
        }
        self.render(buffer,sprite_verticies,sprite_indicies,atlas);
    }
}

pub struct GameObjectBuilder {
    pub graphics: Option<GraphicsType>,
    pub behaviours: Vec<Behaviour>,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pre_rapier_collider: Option<AlcubierreCollider>,
    pub rigid_body: Option<RigidBody>,
}

impl GameObjectBuilder {
    pub fn new() -> GameObjectBuilder {
        GameObjectBuilder {
            graphics: None,
            behaviours: vec![],
            pos_y: 0.0,
            pos_x: 0.0,
            pre_rapier_collider: None,
            rigid_body: None,
        }
    }
    pub fn graphics(mut self, graphics: GraphicsType) -> GameObjectBuilder {
        self.graphics = Some(graphics);
        self
    }
    pub fn rigid_body(mut self, rigid_body: RigidBody) -> GameObjectBuilder {
        self.rigid_body = Some(rigid_body);
        self
    }
    pub fn behaviour(mut self, behaviourPath: &str) -> GameObjectBuilder {
        let data = fs::read_to_string(behaviourPath).expect("Unable to rea behaviour file");
        //
        let engine = Engine::new();
        //TODO: Remove unwrap
        let ast = engine.compile(data).unwrap();

        let behaviour = Behaviour {
            ast: Arc::new(ast),
            scope: Scope::new(),
        };
        self.behaviours.push(behaviour);
        self
    }
    pub fn collider(mut self, collider: AlcubierreCollider) -> GameObjectBuilder {
        self.pre_rapier_collider = Some(collider);
        self
    }
}
