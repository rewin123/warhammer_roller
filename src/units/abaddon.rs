use crate::{unit::Unit, weapon::{RangeWeapon, StandardRangeWeapon, RangeWeaponType, MeeleWeapon, StandardMeeleWeapon}, WahValue};
use log::*;

pub struct Abaddon {
    health : i32,
    name : String,
    range_weapons : Vec<Box<dyn RangeWeapon>>,
    meele_weapons : Vec<Box<dyn MeeleWeapon>>,
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

    fn get_meele_1() -> Box<dyn MeeleWeapon> {
        Box::new(StandardMeeleWeapon {
            damage: WahValue::Fixed(3),
            attack_mul : 1,
            strength: 6 + 3,
            armor_piercing: -4,
            name: "Drach’nyen".to_string(),
        })
    }

    fn get_meele_2() -> Box<dyn MeeleWeapon> {
        Box::new(StandardMeeleWeapon {
            damage: WahValue::Fixed(1),
            attack_mul : 2,
            strength: 6,
            armor_piercing: -4,
            name: "Talon of Horus (melee)".to_string(),
        })
    }
}

impl Default for Abaddon {
    fn default() -> Self {
        
        Self { 
            health: 9, 
            name: "ABADDON THE DESPOILER".to_string(), 
            range_weapons: vec![Abaddon::get_shooting()], 
            meele_weapons: vec![Abaddon::get_meele_1()],
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

    fn add_damage(&mut self, damage : i32, s : i32) -> i32 {
        if self.first_save {
            self.first_save = false;
            0
        } else {
            let mut cur_damage = damage;
            if s >= 2 * self.get_toughness() {
                cur_damage -= 1;
            }
            let start = self.health;
            self.health -= cur_damage;
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

    fn get_ignore_ap(&self) -> i32 {
        1
    }

    fn pretty_report_ballistic_skill(&self, roller : &mut crate::Roller) {
        let weapons = self.get_ranged_weapons();
        for weapon in weapons {
            let attack_count = weapon.get_attack_count().get(roller);
            let mut good_shoot = 0;
            let bs = self.get_ballistic_skill().unwrap();
            for idx in 0..attack_count {
                let roll = roller.d6();
                if roll >= bs {
                    good_shoot += 1;
                } else {

                }
            }

            println!("{} : {} from {}", weapon.get_name(), good_shoot, attack_count);
        }
    }

    fn get_meele_attack_count(&self) -> WahValue {
        WahValue::Fixed(8)
    }

    fn is_in_meele_battle(&self) -> bool {
        true
    }
    
    fn get_meele_weapons(&self) -> &Vec<Box<dyn MeeleWeapon>> {
        &self.meele_weapons
    }
}