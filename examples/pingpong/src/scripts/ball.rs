use alcubierre::game_object::behaviours::EngineView;
use alcubierre::game_object::behaviours::UserBehaviour;
use alcubierre::game_object::GameObject;
use alcubierre::game_object::GameObjectView;
use rapier2d::prelude::{vector, Vector};

#[derive(Clone)]
pub struct BallBehaviour {
}

impl UserBehaviour for BallBehaviour {
    fn game_loop(&mut self, game_object_view: GameObjectView, engine_view: EngineView) {}

    fn loaded(&mut self, engine_view: EngineView, game_object_view: GameObjectView) {
        let rigid_body = engine_view
            .rigid_body_set
            .get_mut(game_object_view.physics.rigid_body_handle.unwrap())
            .unwrap();
        rigid_body.set_linvel(Vector::new(-0.4, 0.0), true);
    }
}
