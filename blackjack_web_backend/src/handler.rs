use crate::model::GameState;
use axum::Json;
use serde_json::json;

pub async fn new_game() -> Json<serde_json::Value> {
    let mut game_state = GameState::new();
    game_state.first_deal();

    Json(json!({
        "status": 200,
        "data": game_state,
        "message": "重新发牌"
    }))
}
