use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum BattleResult {
    Victory,
    Defeat,
    Stalemate,
}

pub struct BattleSystem;

impl BattleSystem {
    pub fn resolve_battle(attacker_power: u32, defender_power: u32) -> BattleResult {
        let mut rng = rand::thread_rng();
        
        // 添加随机因素
        let attacker_luck: f32 = rng.gen_range(0.8..1.2);
        let defender_luck: f32 = rng.gen_range(0.8..1.2);
        
        let attacker_effective = (attacker_power as f32 * attacker_luck) as u32;
        let defender_effective = (defender_power as f32 * defender_luck) as u32;
        
        match attacker_effective.cmp(&defender_effective) {
            std::cmp::Ordering::Greater => BattleResult::Victory,
            std::cmp::Ordering::Less => BattleResult::Defeat,
            std::cmp::Ordering::Equal => BattleResult::Stalemate,
        }
    }
    
    pub fn calculate_damage(attacker_power: u32, defender_defense: u32) -> u32 {
        let base_damage = attacker_power.saturating_sub(defender_defense);
        let mut rng = rand::thread_rng();
        let multiplier = rng.gen_range(0.5..1.5);
        
        (base_damage as f32 * multiplier) as u32
    }
    
    pub fn simulate_battle(mut attacker_power: u32, mut defender_power: u32) -> (BattleResult, u32, u32) {
        let mut rounds = 0;
        
        while attacker_power > 0 && defender_power > 0 && rounds < 10 {
            let damage_to_defender = Self::calculate_damage(attacker_power, defender_power / 2);
            let damage_to_attacker = Self::calculate_damage(defender_power, attacker_power / 2);
            
            defender_power = defender_power.saturating_sub(damage_to_defender);
            attacker_power = attacker_power.saturating_sub(damage_to_attacker);
            
            rounds += 1;
        }
        
        let result = if attacker_power > defender_power {
            BattleResult::Victory
        } else if defender_power > attacker_power {
            BattleResult::Defeat
        } else {
            BattleResult::Stalemate
        };
        
        (result, attacker_power, defender_power)
    }
}