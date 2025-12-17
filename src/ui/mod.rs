mod input;

pub use input::{get_player_action, get_input};
use crate::game::GameState;
use colored::*;

pub fn display_status(state: &GameState) {
    println!("\n{}", "=".repeat(50).bright_blue());
    println!("{}", "《逆天而行》".bright_yellow().bold());
    println!("{}", "=".repeat(50).bright_blue());
    
    // 玩家信息
    println!("\n{}: {}", "玩家".bright_green(), state.player.name.bold());
    println!("{}: {}/100", "生命值".red(), state.player.health);
    println!("{}: {} 回合", "当前回合".cyan(), state.turn);
    println!("{}: {:?}", "游戏阶段".magenta(), state.phase);
    
    // 属性
    println!("\n{}", "【属性】".bright_cyan());
    println!("力量: {} | 智力: {} | 魅力: {} | 野心: {} | 运气: {} | 逆天改命值: {} | 色欲: {} | 神性: {} | 黑暗: {} | 敏捷: {} | 体力: {}",
        state.player.stats.strength,
        state.player.stats.intelligence,
        state.player.stats.charisma,
        state.player.stats.ambition,
        state.player.stats.luck,
        state.player.stats.nts,
        state.player.stats.lust,
        state.player.stats.divinity,
        state.player.stats.darkness,
        state.player.stats.agility,
        state.player.stats.stamina,
    );
    
    // 资源
    println!("\n{}", "【资源】".bright_cyan());
    println!("金币: {} | 兵力: {} | 影响力: {} | 科技: {}",
        state.player.resources.gold,
        state.player.resources.manpower,
        state.player.resources.influence,
        state.player.resources.technology
    );
    
    // 势力
    println!("\n{}", "【势力】".bright_cyan());
    println!("追随者: {} | 领地: {} | 世界抵抗: {}",
        state.player.followers,
        state.player.territories.len(),
        state.world_power
    );
    
    if !state.player.territories.is_empty() {
        println!("领地列表: {}", 
            state.player.territories.join(", ").bright_green()
        );
    }
    
    println!("{}", "-".repeat(50).dimmed());
}

pub fn display_ending(state: &GameState) {
    println!("\n{}", "=".repeat(50).bright_yellow());
    println!("{}", "游戏结束".bold());
    println!("{}", "=".repeat(50).bright_yellow());
    
    let score = calculate_score(state);
    
    println!("\n最终统计:");
    println!("玩家: {}", state.player.name);
    println!("总回合数: {}", state.turn);
    println!("最终领地数: {}", state.player.territories.len());
    println!("最终追随者: {}", state.player.followers);
    println!("关键决策数: {}", state.decisions.len());
    println!("世界抵抗力量: {}", state.world_power);
    
    println!("\n{}", "最终评分:".bright_green());
    match score {
        90..=100 => println!("{} - 逆天成神！", "SSS".bright_yellow()),
        80..=89 => println!("{} - 世界征服者", "SS".bright_green()),
        70..=79 => println!("{} - 帝国建立者", "S".green()),
        60..=69 => println!("{} - 一方霸主", "A".cyan()),
        50..=59 => println!("{} - 小有成就", "B".blue()),
        _ => println!("{} - 还需努力", "C".dimmed()),
    }
    
    if !state.decisions.is_empty() {
        println!("\n{}", "关键决策回顾:".bright_magenta());
        for (i, decision) in state.decisions.iter().enumerate() {
            println!("{}. {}", i + 1, decision);
        }
    }
}

fn calculate_score(state: &GameState) -> u32 {
    let mut score = 0;
    
    // 领地得分
    score += state.player.territories.len() as u32 * 10;
    
    // 追随者得分
    score += (state.player.followers / 10) as u32;
    
    // 属性得分
    score += state.player.stats.strength;
    score += state.player.stats.intelligence;
    score += state.player.stats.charisma;
    score += state.player.stats.ambition;
    
    // 资源得分
    score += (state.player.resources.gold / 100) as u32;
    score += state.player.resources.technology;
    
    // 世界抵抗扣除
    score = score.saturating_sub(state.world_power / 10);
    
    // 回合数奖励（越快越好）
    if state.turn < 20 {
        score += 30;
    } else if state.turn < 30 {
        score += 10;
    }
    
    score.min(100)
}

pub fn display_help() {
    println!("\n{}", "可用命令:".bright_cyan());
    println!("recruit  - 招募追随者");
    println!("conquer  - 征服地区");
    println!("status   - 查看状态");
    println!("save     - 保存游戏");
    println!("load     - 加载游戏");
    println!("end      - 结束回合");
    println!("quit     - 退出游戏");
    println!("help     - 显示帮助");
}

pub fn display_message(message: &str, message_type: MessageType) {
    match message_type {
        MessageType::Info => println!("{}", message.bright_blue()),
        MessageType::Success => println!("{}", message.bright_green()),
        MessageType::Warning => println!("{}", message.bright_yellow()),
        MessageType::Error => println!("{}", message.bright_red()),
        MessageType::Event => println!("{}", message.bright_magenta()),
    }
}

pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
    Event,
}