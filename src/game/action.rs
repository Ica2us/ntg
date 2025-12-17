use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PlayerAction {
    RecruitFollowers,
    TrainTroops,
    ConquerRegion(u32),
    DevelopTechnology,
    FormAlliance,
    WarCampaign,
    EstablishEmpire,
    ExpandTerritory,
    FinalBattle,
    WeakenEnemy,
    GatherForces,
    AcceptSurrender,
    EndTurn,
    ViewStatus,
    SaveGame,
    LoadGame,
    Quit,
}

impl PlayerAction {
    pub fn from_input(input: &str, _phase: &crate::game::phase::GamePhase) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "recruit" | "1" => Some(PlayerAction::RecruitFollowers),
            "conquer" | "2" => Some(PlayerAction::ConquerRegion(1)), // 简化，实际需要参数
            "end" | "quit" => Some(PlayerAction::EndTurn),
            "status" => Some(PlayerAction::ViewStatus),
            "save" => Some(PlayerAction::SaveGame),
            "load" => Some(PlayerAction::LoadGame),
            _ => None,
        }
    }
}