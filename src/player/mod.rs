mod character;
mod inventory;

pub use character::Character;
pub use character::Stats;
pub use character::Resources;
pub use inventory::Inventory;

// 如果需要在模块内共享私有项，可以在这里声明
//use crate::utils::helpers;