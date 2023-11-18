use crate::equipment::equipment_type::EquipmentType;

pub struct CharacterInventory {
    pub equipment: Vec<EquipmentType>,
}

impl CharacterInventory {
    pub fn new() -> Self {
        CharacterInventory {
            equipment: Vec::<EquipmentType>::new(),
        }
    }

    pub fn add_item(&mut self, item: EquipmentType) {
        self.equipment.push(item);
    }

    pub fn remove_item(&mut self, item: EquipmentType) {
        self.equipment.pop();
    }
}
