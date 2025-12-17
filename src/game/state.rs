use serde::{Serialize, Deserialize};
use crate::player::{Character};
use crate::world::{World, Region};
use crate::combat::BattleResult;
use super::phase::GamePhase;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub player: Character,
    pub world: World,
    pub phase: GamePhase,
    pub turn: u32,
    pub world_power: u32,
    pub decisions: Vec<String>,
}

impl GameState {
    pub fn new(player_name: &str) -> Self {
        GameState {
            player: Character::new(player_name),
            world: World::new(),
            phase: GamePhase::Beginning,
            turn: 1,
            world_power: 100,
            decisions: Vec::new(),
        }
    }
    
    pub fn is_game_over(&self) -> bool {
        self.player.health <= 0 || self.world_power == 0 || self.turn > 30
    }
    
    pub fn end_turn(&mut self) {
        self.turn += 1;
        
        // 每回合自动恢复
        if self.player.health < 100 {
            self.player.health = (self.player.health + 5).min(100);
        }
        
        // 每回合收入
        self.player.resources.gold += self.player.territories.len() as u32 * 10;
    }
    
    pub fn recruit_followers(&mut self) -> Result<(), String> {
        if self.player.resources.gold < 5 {
            return Err("金币不足".to_string());
        }
        
        self.player.resources.gold -= 5;
        let new_followers = rand::thread_rng().gen_range(5..=15);
        self.player.followers += new_followers;
        
        Ok(())
    }
    
    pub fn conquer_region(&mut self, region_id: u32) -> Result<(), String> {
        if let Some(region) = self.world.get_region(region_id) {
            let battle_result = self.resolve_battle(region);
            
            match battle_result {
                BattleResult::Victory => {
                    self.player.territories.push(region.name.clone());
                    self.world_power = self.world_power.saturating_sub(10);
                    self.decisions.push(format!("征服了{}", region.name));
                    Ok(())
                }
                BattleResult::Defeat => {
                    let damage = rand::thread_rng().gen_range(10..=30);
                    self.player.health -= damage;
                    Err(format!("征服失败，受到{}点伤害", damage))
                }
                BattleResult::Stalemate => {
                    Err("战斗陷入僵局".to_string())
                }
            }
        } else {
            Err("地区不存在".to_string())
        }
    }
    
    fn resolve_battle(&self, region: &Region) -> BattleResult {
        use crate::combat::BattleSystem;
        
        let player_power = self.player.calculate_battle_power();
        let region_power = region.difficulty * 10;
        
        BattleSystem::resolve_battle(player_power, region_power)
    }
    
    pub fn trigger_random_event(&mut self) {
        use rand::Rng;
        let event_type = rand::thread_rng().gen_range(0..4);
        
        match event_type {
            0 => {
                println!("发现宝藏!获得50金币");
                self.player.resources.gold += 50;
            }
            1 => {
                println!("遭遇袭击,失去10名追随者");
                self.player.followers = self.player.followers.saturating_sub(10);
            }
            2 => {
                println!("天降祥瑞，所有属性+1");
                self.player.stats.strength += 1;
                self.player.stats.intelligence += 1;
                self.player.stats.charisma += 1;
            }
            3 => {
                println!("领地叛乱，需要处理");
                self.world_power += 5;
            }
            _ => {}
        }
    }
    
    pub fn save(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(self)?;
        std::fs::write(filename, serialized)?;
        Ok(())
    }
    
    pub fn load(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(filename)?;
        let state = serde_json::from_str(&data)?;
        Ok(state)
    }
}