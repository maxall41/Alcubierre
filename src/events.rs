use std::time::Duration;
use rapier2d::geometry::ColliderHandle;
use crate::audio::basic::AudioSource;
use crate::Engine;
use crate::game_object::behaviours::EngineView;
use crate::game_object::{GameObject, GameObjectIPC, GameObjectView};

pub struct PullGameObjectRequest {
    pub(crate) collider_handle: ColliderHandle,
    pub(crate) callback: Box<dyn Fn(EngineView,&GameObject)>,
}

pub enum EngineEvent {
    SwitchToScene(String),
    SetDatamapValue((String, String)),
    InsertDatamapValue((String, String)),
    RemoveDatamapValue(String),
    PlaySound(AudioSource),
    PullGameObject(PullGameObjectRequest)
}

impl Engine {
    pub(crate) fn handle_events(&mut self) {
        let packet = self.event_rx.try_recv();
        match packet {
            Ok(event) => match event {
                EngineEvent::SwitchToScene(scene) => {
                    self.set_current_scene(scene);
                }
                EngineEvent::PullGameObject(req) => {
                    // Retrieve GameObject
                    let scene = self.active_scene.as_mut().unwrap();
                    let object_id = scene.collider_set.get(req.collider_handle).unwrap();
                    let game_object = scene.game_objects.get(object_id.user_data as usize).unwrap();
                    // Return GameObject to sender
                    (req.callback)(EngineView {
                        rigid_body_set: &mut scene.rigid_body_set,
                        narrow_phase: &mut scene.narrow_phase_collision,
                        event_tx: &mut self.event_tx,
                        keys_pressed: &mut self.keys_pressed,
                        key_locks: &mut self.key_locks,
                        query_pipeline: &mut self.query_pipeline,
                        collider_set: &mut scene.collider_set,
                        frame_delta: &Duration::from_millis(5) //TODO
                    },game_object)
                }
                EngineEvent::SetDatamapValue((var, val)) => {
                    *self
                        .active_scene
                        .as_mut()
                        .unwrap()
                        .data_map
                        .get_mut(&var)
                        .unwrap() = val;
                }
                EngineEvent::InsertDatamapValue((var, val)) => {
                    self.active_scene
                        .as_mut()
                        .unwrap()
                        .data_map
                        .insert(var, val);
                }
                EngineEvent::RemoveDatamapValue(var) => {
                    self.active_scene.as_mut().unwrap().data_map.remove(&var);
                }
                EngineEvent::PlaySound(source) => {
                    self.play_audio(source);
                }
            },
            Err(_e) => {
                // panic!("{}",e); //TODO: Handle
            }
        }
    }
}
