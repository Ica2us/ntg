use std::io;
use colored::Colorize;

use crate::game::PlayerAction;
use crate::game::GameState;

pub fn get_player_action(state: &GameState) -> PlayerAction {
    println!("\n{}", "请输入行动命令 (输入 'help' 查看帮助):".on_bright_cyan());
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        
        let trimmed = input.trim().to_lowercase();
        
        match trimmed.as_str() {
            "help" => {
                super::display_help();
                continue;
            }
            "status" => {
                super::display_status(state);
                continue;
            }
            "quit" | "exit" => return PlayerAction::Quit,
            "save" => return PlayerAction::SaveGame,
            "load" => return PlayerAction::LoadGame,
            "recruit" | "1" => return PlayerAction::RecruitFollowers,
            "conquer" | "2" => {
                println!("请输入要征服的地区ID (1-3):");
                if let Ok(region_id) = get_input(1..=3) {
                    return PlayerAction::ConquerRegion(region_id);
                } else {
                    println!("无效的地区ID");
                    continue;
                }
            }
            "end" | "3" => return PlayerAction::EndTurn,
            _ => {
                println!("未知命令，请输入 'help' 查看可用命令");
                continue;
            }
        }
    }
}

pub fn get_input<T>(range: std::ops::RangeInclusive<T>) -> Result<T, String>
where
    T: std::str::FromStr + PartialOrd + Copy,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    
    match input.trim().parse() {
        Ok(num) if range.contains(&num) => Ok(num),
        Ok(_) => Err("数值超出范围".to_string()),
        Err(_) => Err("无效的输入".to_string()),
    }
}