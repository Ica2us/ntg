use rand::Rng;

pub fn format_number(num: u32) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f32 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f32 / 1_000.0)
    } else {
        num.to_string()
    }
}

pub fn random_name() -> String {
    let first_names = vec!["赵", "钱", "孙", "李", "周", "吴", "郑", "王"];
    let last_names = vec!["天", "地", "玄", "黄", "宇", "宙", "洪", "荒"];
    
    let mut rng = rand::thread_rng();
    let first = first_names[rng.gen_range(0..first_names.len())];
    let last = last_names[rng.gen_range(0..last_names.len())];
    
    format!("{}{}", first, last)
}

pub fn roll_dice(sides: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=sides)
}

pub fn calculate_modifier(attribute: u32) -> i32 {
    ((attribute as i32) - 10) / 2
}

pub fn chance(percent: u32) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100) <= percent
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}