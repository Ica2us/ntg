use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Inventory {
    items: HashMap<String, u32>,
    equipment: Vec<String>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: HashMap::new(),
            equipment: Vec::new(),
        }
    }
    
    pub fn add_item(&mut self, item: &str, quantity: u32) {
        *self.items.entry(item.to_string()).or_insert(0) += quantity;
    }
    
    pub fn remove_item(&mut self, item: &str, quantity: u32) -> bool {
        if let Some(count) = self.items.get_mut(item) {
            if *count >= quantity {
                *count -= quantity;
                if *count == 0 {
                    self.items.remove(item);
                }
                return true;
            }
        }
        false
    }
    
    pub fn has_item(&self, item: &str) -> bool {
        self.items.contains_key(item)
    }
    
    pub fn equip(&mut self, item: &str) {
        if !self.equipment.contains(&item.to_string()) {
            self.equipment.push(item.to_string());
        }
    }
    
    pub fn get_items(&self) -> &HashMap<String, u32> {
        &self.items
    }
}