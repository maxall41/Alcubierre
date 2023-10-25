fn awake() {
    let score = "0";
    1
}


fn update(delta) {
    const SPEED = 0.1;
    let score_incr = 0;
    if Input.is_key_down("d") {
        self.rigid_body_pos_x = self.rigid_body_pos_x + SPEED * delta;
        score_incr = score_incr + 1;
    }
    if Input.is_key_down("a") {
            self.rigid_body_pos_x = self.rigid_body_pos_x - SPEED * delta;

            score_incr = score_incr + 1;
    }
    if Input.is_key_down("w") {
            self.rigid_body_pos_y = self.rigid_body_pos_y + SPEED * delta;

            score_incr = score_incr + 1;
    }
    if Input.is_key_down("s") {
           self.rigid_body_pos_y = self.rigid_body_pos_y - SPEED * delta;

           score_incr = score_incr + 1;
    }
    score = (parse_int(score) + score_incr).to_string();
    if score_incr > 0 {
        engine.set_datamap_value("ScoreValue",score.to_string());
    }
    self
}