use serde::{Serialize, Deserialize};
use crate::world::faction::Faction;

#[derive(Serialize, Deserialize, Clone)]
pub struct Region {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub difficulty: u32,
    pub resources: Vec<String>,
    pub controlled_by: Option<u32>, // Faction ID
}

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    regions: Vec<Region>,
    factions: Vec<Faction>,
}

impl World {
    pub fn new() -> Self {
        let regions = vec![
            Region {
                id: 1,
                name: "新手村".to_string(),
                description: "一个宁静的小村庄，适合新手起步".to_string(),
                difficulty: 1,
                resources: vec!["木材".to_string(), "粮食".to_string()],
                controlled_by: None,
            },
            Region {
                id: 2,
                name: "黑风寨".to_string(),
                description: "土匪盘踞的山寨，危险但富有资源".to_string(),
                difficulty: 3,
                resources: vec!["黄金".to_string(), "武器".to_string()],
                controlled_by: Some(1), // 土匪势力
            },
            Region {
                id: 3,
                name: "青龙城".to_string(),
                description: "繁华的商业城市，守卫森严".to_string(),
                difficulty: 5,
                resources: vec!["丝绸".to_string(), "香料".to_string(), "黄金".to_string()],
                controlled_by: Some(2), // 城主势力
            },
        ];
        
        let factions = vec![
            Faction::new(1, "土匪帮".to_string()),
            Faction::new(2, "青龙城主".to_string()),
        ];
        
        World { regions, factions }
    }
    pub fn get_region(&self, id: u32) -> Option<&Region> {
        self.regions.iter().find(|r| r.id == id)
    }
    
    pub fn get_available_regions(&self, player_power: u32) -> Vec<&Region> {
        self.regions
            .iter()
            .filter(|r| r.difficulty as u32 <= player_power / 10)
            .collect()
    }
    
    pub fn conquer_region(&mut self, region_id: u32, conqueror_id: u32) -> bool {
        if let Some(region) = self.regions.iter_mut().find(|r| r.id == region_id) {
            region.controlled_by = Some(conqueror_id);
            true
        } else {
            false
        }
    }
}
