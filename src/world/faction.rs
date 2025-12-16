use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Faction {
    pub id: u32,
    pub name: String,
    pub power: u32,
    pub relation: i32, // 对玩家的关系，-100到100
    pub regions: Vec<u32>,
}

impl Faction{
    pub fn new(id: u32, name: String) -> Self {
        Faction {
            id,
            name,
            power: 50,
            relation: 0,
            regions: Vec::new(),
        }
    }

    pub fn add_region(&mut self, region_id: u32) {
        if !self.regions.contains(&region_id) {
            self.regions.push(region_id);
            self.power += 10;
        }
    }

     pub fn remove_region(&mut self, region_id: u32) {
        self.regions.retain(|&id| id != region_id);
        self.power = self.power.saturating_sub(10);
    }
    
    pub fn modify_relation(&mut self, delta: i32) {
        self.relation = (self.relation + delta).clamp(-100, 100);
    }
    
    pub fn is_hostile(&self) -> bool {
        self.relation < -30
    }

    pub fn is_friendly(&self) -> bool {
        self.relation > 30
    }
    
}