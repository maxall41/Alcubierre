use crate::audio::basic::AudioSource;
use crate::Engine;

pub enum EngineEvent {
    SwitchToScene(String),
    SetDatamapValue((String, String)),
    InsertDatamapValue((String, String)),
    RemoveDatamapValue(String),
    PlaySound(AudioSource),
}

impl Engine {
    pub(crate) fn handle_events(&mut self) {
        let packet = self.event_rx.try_recv();
        match packet {
            Ok(event) => match event {
                EngineEvent::SwitchToScene(scene) => {
                    self.set_current_scene(scene);
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
