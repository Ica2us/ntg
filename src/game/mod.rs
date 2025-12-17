mod state;
mod phase;
mod action;

pub use state::GameState;
pub use phase::GamePhase;
pub use action::PlayerAction;
//use crate::player::Character;
//use crate::world::World;
use crate::ui;

use std::error::Error;

pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new(player_name: &str) -> Self {
        Game {
            state: GameState::new(player_name),
        }
    }
    
    pub fn run(&mut self) {
        self.game_loop();
    }
    
    fn game_loop(&mut self) {
        while !self.state.is_game_over() {
            // 显示游戏状态
            ui::display_status(&self.state);
            
            // 获取玩家行动
            let action = ui::get_player_action(&self.state);
            
            // 执行行动
            self.execute_action(action);
            
            // 处理世界事件
            self.process_world_events();
            
            // 回合结束
            self.state.end_turn();
            
            // 检查阶段转换
            self.check_phase_transition();
        }
        
        // 显示结局
        self.show_ending();
    }
    
    fn execute_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::RecruitFollowers => {
                if let Err(e) = self.state.recruit_followers() {
                    println!("招募失败: {}", e);
                }
            }
            PlayerAction::ConquerRegion(region_id) => {
                if let Err(e) = self.state.conquer_region(region_id) {
                    println!("征服失败: {}", e);
                }
            }
            PlayerAction::EndTurn => {
                // 结束回合已在循环中处理
            }
            // ... 其他行动
            _ => println!("此功能尚未实现"),
        }
    }
    
    fn process_world_events(&mut self) {
        // 处理随机事件
        if rand::random::<f32>() < 0.3 {
            self.state.trigger_random_event();
        }
    }
    
    fn check_phase_transition(&mut self) {
        let new_phase = match self.state.turn {
            0..=5 => GamePhase::Beginning,
            6..=15 => GamePhase::Rise,
            16..=25 => GamePhase::Empire,
            _ => GamePhase::Ending,
        };
        
        if self.state.phase != new_phase {
            self.state.phase = new_phase;
            println!("进入新阶段: {:?}", new_phase);
        }
    }
    
    fn show_ending(&self) {
        ui::display_ending(&self.state);
    }
    
    // 保存和加载功能
    pub fn save(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        self.state.save(filename)
    }
    
    pub fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let state = GameState::load(filename)?;
        Ok(Game { state })
    }
}