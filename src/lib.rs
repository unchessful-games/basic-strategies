use engine_trait::{
    async_trait,
    server_types::EngineInfo,
    shakmaty::{san::San, Chess, Move, Position},
    Engine, InfallibleError,
};
use serde::{Deserialize, Serialize};

pub struct Random;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RandomStatusInfo {
    pub available_moves: Vec<String>,
    pub chosen_move_idx: usize,
}

#[async_trait]
impl Engine for Random {
    type State = ();

    type StatusInfo = RandomStatusInfo;

    type Error = InfallibleError;

    fn get_info() -> EngineInfo<Self> {
        EngineInfo {
            id: "random_move".to_string(),
            description: "Plays a random available legal move".to_string(),
            initial_state: (),
        }
    }

    async fn propose_move(
        &mut self,
        rand: u64,
        _current_state: &Self::State,
        current_position: &Chess,
    ) -> Result<(Move, Self::StatusInfo), Self::Error> {
        let available_moves: Vec<Move> = current_position.legal_moves().iter().cloned().collect();
        let chosen_move_idx = (rand as usize) % available_moves.len();
        Ok((
            available_moves[chosen_move_idx].clone(),
            RandomStatusInfo {
                available_moves: available_moves
                    .iter()
                    .map(|v| San::from_move(current_position, &v).to_string())
                    .collect(),
                chosen_move_idx,
            },
        ))
    }

    async fn propose_move_without_info(
        &mut self,
        rand: u64,
        _current_state: &Self::State,
        current_position: &Chess,
    ) -> Result<Move, Self::Error> {
        let moves = current_position.legal_moves();
        let chosen = moves[(rand as usize) % moves.len()].clone();
        Ok(chosen)
    }

    async fn observe_move(
        &mut self,
        _rand: u64,
        _state: &mut Self::State,
        _move_taken: &Move,
        _position_after: &Chess,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
