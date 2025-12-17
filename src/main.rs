use ntg::game;
use std::io;

fn main() {
    println!("+++<*重生之鳜鱼逆天改命日穿水卡一*>+++");
    println!("请输入玩家名称：");
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).expect("读取输入失败");
    let player_name = player_name.trim();
    let mut game = game::Game::new(player_name);
    game.run();
}
