use super::inventory::Inventory;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    //title: String, // 称号：凡人→修士→大能→圣人→逆天者

    pub stats: Stats,

    // 资源
    pub resources: Resources,
    pub territories: Vec<String>,
    pub inventory: Inventory,

    pub followers: u32, // 追随者

    //lovers: Vec<String>,      // 红颜/知己TODO

    // 状态
    pub health: i32,
    cultivation_level: CultivationLevel,
    karma: i32, // 业力/声望（正邪）
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub nts: u32,          // 鳜鱼逆天改命值
    pub strength: u32,     // 力量
    pub agility: u32,      // 敏捷
    pub intelligence: u32, // 智力
    pub charisma: u32,     // 魅力
    pub ambition: u32,     // 野心
    pub lust: u32,         // 色欲
    pub stamina: u32,      // 体力
    pub luck: u32,         // 运气
    pub darkness: u32,     // 黑暗面
    pub divinity: u32,     // 神性
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Resources {
    pub gold: u32,
    pub manpower: u32,
    pub influence: u32,
    pub technology: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum CultivationLevel {
    Mortal,         // 凡人
    QiCondensation, // 炼气
    Foundation,     // 筑基
    GoldenCore,     // 金丹
    NascentSoul,    // 元婴
    Deity,          // 化神
    God,            // 真神
    UniverseRuler,  // 宇宙主宰
}

impl Character {
    pub fn new(name: &str) -> Self {
        Character {
            name: name.to_string(),
            stats: Stats {
                strength: 5,
                intelligence: 5,
                charisma: 5,
                ambition: 5,
                luck: 5,
                nts: 0,
                agility: 5,
                lust: 5,
                stamina: 5,
                darkness: 0,
                divinity: 0,
            },
            resources: Resources {
                gold: 10,
                manpower: 0,
                influence: 10,
                technology: 0,
            },
            health: 100,
            followers: 0,
            karma: 0,
            cultivation_level: CultivationLevel::Mortal,
            territories: Vec::new(),
            inventory: Inventory::new(),
        }
    }

    pub fn calculate_battle_power(&self) -> u32 {
        self.stats.strength * 3 + self.followers / 2 + self.resources.manpower
    }
    pub fn apply_trait(&mut self, trait_name: &str) {
        match trait_name {
            "天生领袖" => {
                self.stats.charisma += 3;
                self.stats.ambition += 2;
            }
            "冷酷无情" => {
                self.stats.strength += 2;
                self.stats.ambition += 3;
            }
            "远见卓识" => {
                self.stats.intelligence += 3;
                self.resources.technology += 10;
            }
            _ => {}
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}


