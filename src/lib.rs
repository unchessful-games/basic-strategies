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

pub struct MinOpptMoves;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MinOpptMovesInfo {
    pub enemy_moves_after: usize,
}

#[async_trait]
impl Engine for MinOpptMoves {
    type State = ();

    type StatusInfo = MinOpptMovesInfo;

    type Error = InfallibleError;

    fn get_info() -> EngineInfo<Self> {
        EngineInfo {
            id: "min_oppt_moves".to_string(),
            description: "Plays the move to which the opponent has the fewest legal responses"
                .to_string(),
            initial_state: (),
        }
    }

    async fn propose_move(
        &mut self,
        _rand: u64,
        _current_state: &Self::State,
        current_position: &Chess,
    ) -> Result<(Move, Self::StatusInfo), Self::Error> {
        let moves = current_position.legal_moves();
        let mut best_move = moves[0].clone();
        let score_move = |m| {
            let mut new_position = current_position.clone();
            new_position.play_unchecked(&m);
            new_position.legal_moves().len()
        };

        let mut best_move_score = score_move(best_move.clone());
        for new_move in moves.iter().skip(1) {
            let score = score_move(new_move.clone());
            if score < best_move_score {
                best_move_score = score;
                best_move = new_move.clone();
            }
        }

        Ok((
            best_move,
            MinOpptMovesInfo {
                enemy_moves_after: best_move_score,
            },
        ))
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

pub struct LexicographicallyFirst;

#[async_trait]
impl Engine for LexicographicallyFirst {
    type State = ();

    type StatusInfo = ();

    type Error = InfallibleError;

    fn get_info() -> EngineInfo<Self> {
        EngineInfo {
            id: "lexicographically_first".to_string(),
            description: "Plays the move whose standard notation is lexicographically first."
                .to_string(),
            initial_state: (),
        }
    }

    async fn propose_move(
        &mut self,
        _rand: u64,
        _current_state: &Self::State,
        current_position: &Chess,
    ) -> Result<(Move, Self::StatusInfo), Self::Error> {
        let mut moves: Vec<_> = current_position
            .legal_moves()
            .into_iter()
            .map(|v| (San::from_move(current_position, &v).to_string(), v))
            .collect();
        moves.sort_by(|a, b| a.0.cmp(&b.0));

        Ok((moves.into_iter().next().unwrap().1, ()))
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
