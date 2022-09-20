use crate::{unit::Unit, weapon::{RangeWeapon, StandardRangeWeapon, RangeWeaponType}, WahValue};


pub struct Abaddon {
    health : i32,
    name : String,
    range_weapons : Vec<Box<dyn RangeWeapon>>,
    first_save : bool,
    phase_end_health : i32
}

impl Abaddon {
    fn get_shooting() -> Box<dyn RangeWeapon> {
        Box::new(StandardRangeWeapon {
            damage: WahValue::Fixed(2),
            tp: RangeWeaponType::Assault,
            shoot_count: WahValue::Fixed(4),
            strength: 5,
            armor_piercing: -1,
            name: "Talon of Horus (shooting)".to_string(),
        })
    }
}

impl Default for Abaddon {
    fn default() -> Self {
        
        Self { 
            health: 9, 
            name: "ABADDON THE DESPOILER".to_string(), 
            range_weapons: vec![Abaddon::get_shooting()], 
            first_save: true, 
            phase_end_health: 9 - 3
        }
    }
}

impl Unit for Abaddon {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_ballistic_skill(&self) -> Option<i32> {
        Some(2)
    }

    fn get_weapon_skill(&self) -> Option<i32> {
        Some(2)
    }

    fn get_save(&self) -> i32 {
        2
    }

    fn get_strength(&self) -> i32 {
        6
    }

    fn get_toughness(&self) -> i32 {
        6
    }

    fn get_ranged_weapons(&self) -> &Vec<Box<dyn crate::weapon::RangeWeapon>> {
        &self.range_weapons
    }

    fn get_involve(&self) -> i32 {
        4
    }

    fn add_damage(&mut self, damage : i32) -> i32 {
        if self.first_save {
            self.first_save = false;
            0
        } else {
            let start = self.health;
            self.health -= damage;
            self.health = self.health.max(self.phase_end_health);
            start - self.health
        }
    }

    fn next_move(&mut self) {
        self.first_save = true;
    }

    fn set_phase(&mut self, phase : &crate::unit::GamePhase) {
        self.phase_end_health = self.health - 3;
    }
}