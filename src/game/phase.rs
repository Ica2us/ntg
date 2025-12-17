use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy,Debug, PartialEq)]
pub enum GamePhase {
    Beginning,
    Rise,
    Empire,
    Ending,
}

impl GamePhase {
    pub fn get_description(&self) -> &str {
        match self {
            GamePhase::Beginning => "逆天开局：你将从底层开始你的逆天之路",
            GamePhase::Rise => "崛起之路：建立势力，扩大影响",
            GamePhase::Empire => "帝国建立：建立属于你的帝国",
            GamePhase::Ending => "最终决战：征服全世界，逆天成神",
        }
    }
    
    pub fn get_available_actions(&self) -> Vec<&str> {
        match self {
            GamePhase::Beginning => vec![
                "接受传承",
                "探索遗迹",
                "加入组织",
                "独自修炼",
            ],
            GamePhase::Rise => vec![
                "招募追随者",
                "征服地区",
                "发展经济",
                "建立联盟",
            ],
            GamePhase::Empire => vec![
                "发动战争",
                "发展科技",
                "建立制度",
                "扩张领土",
            ],
            GamePhase::Ending => vec![
                "最终决战",
                "削弱敌人",
                "集结力量",
                "接受投降",
            ],
        }
    }
}