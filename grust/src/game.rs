use crate::ball::Ball;
use godot::classes::{INode, Label, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    #[export]
    p1_score: i32,
    #[export]
    p2_score: i32,
    #[export]
    p1_label_path: NodePath,
    #[export]
    p2_label_path: NodePath,

    base: Base<Node>,
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            p1_score: 0,
            p2_score: 0,
            p1_label_path: NodePath::from(""),
            p2_label_path: NodePath::from(""),
            base,
        }
    }
}

#[godot_api]
impl GameManager {
    #[func]
    pub fn _on_player_2_score(&mut self, body: Gd<Node>) {
        if let Ok(mut ball) = body.try_cast::<Ball>() {
            self.p2_score += 1;
            self.update_score_ui();
            ball.bind_mut().reset_ball();
        }
    }

    #[func]
    pub fn _on_player_1_score(&mut self, body: Gd<Node>) {
        if let Ok(mut ball) = body.try_cast::<Ball>() {
            self.p1_score += 1;
            self.update_score_ui();
            ball.bind_mut().reset_ball();
        }
    }

    fn update_score_ui(&mut self) {
        let base = self.base();

        if let Some(node) = base.get_node_or_null(&self.p1_label_path.clone()) {
            if let Ok(mut label) = node.try_cast::<Label>() {
                let score = self.p1_score.to_string();
                label.set_text(&score);
            } else {
                godot_error!("Error: p1_score Node is not `Label` type.");
            }
        } else {
            godot_error!(
                "Error: No node found for P1 Label (Path: {})",
                self.p1_label_path
            );
        }

        if let Some(node) = base.get_node_or_null(&self.p2_label_path.clone()) {
            if let Ok(mut label) = node.try_cast::<Label>() {
                let score = self.p2_score.to_string();
                label.set_text(&score);
            } else {
                godot_error!("Error: p2_score Node is not `Label` type.");
            }
        } else {
            godot_error!(
                "Error: No node found for P1 Label (Path: {})",
                self.p2_label_path
            );
        }
    }
}
