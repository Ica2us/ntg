use ntg::game;
use std::io;

fn main() {
    println!("+++<*重生之鳜鱼逆天改命日穿水卡一*>+++");
    println!("请输入存档文件路径：");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("读取输入失败");
    let path = path.trim(); // 去除换行符   
    let mut game = Game::load_from_file(path).expect("加载存档失败");
    game.start();
}
