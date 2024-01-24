pub struct FieldEnergy {
    field_energy_count: u32,

}

impl FieldEnergy {
    pub fn use_field_energy(&mut self, num: i32) {
        self.field_energy_count -= num;
    }

    pub fn plus_field_energy(&mut self, num: i32) {
        self.field_energy_count += num;
    }

}