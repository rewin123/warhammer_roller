use crate::unit::Unit;
use crate::{RolledValue, WahValue};
use crate::weapon::{RangeWeapon, RangeWeaponType, StandardRangeWeapon, MeeleWeapon};

pub struct FortressOfRedemption {
    health : i32,
    weapons : Vec<Box<dyn RangeWeapon>>,
    name : String,
    meele_weapons : Vec<Box<dyn MeeleWeapon>>
}

impl FortressOfRedemption {
    fn get_fragstorm_missle() -> Box<dyn RangeWeapon> {
        Box::new(StandardRangeWeapon {
            damage: WahValue::Fixed(1),
            tp: RangeWeaponType::Heavy,
            shoot_count: WahValue::Rolled(RolledValue {d6 : 2, d3 : 0, add : 0}),
            strength: 4,
            armor_piercing: -1,
            name: "Fragstorm missile".to_string()
        })
    }

    fn get_krakstorm_missle() -> Box<dyn RangeWeapon> {
        Box::new(StandardRangeWeapon {
            damage: WahValue::Fixed(3),
            tp: RangeWeaponType::Heavy,
            shoot_count: WahValue::Rolled(RolledValue {d6 : 1, d3 : 0, add : 0}),
            strength: 8,
            armor_piercing: -8,
            name: "Krakstorm missile".to_string()
        })
    }
}

impl Default for FortressOfRedemption {
    fn default() -> Self {
        let mut weapons : Vec<Box<dyn RangeWeapon>> = vec![
            Box::new(StandardRangeWeapon {
                damage: WahValue::Fixed(2),
                tp: RangeWeaponType::Heavy,
                shoot_count: WahValue::Fixed(3 * 4),
                strength: 5,
                armor_piercing: -1,
                name: "Heavy bolter".to_string()
            }),
            Box::new(StandardRangeWeapon {
                damage: WahValue::Rolled(RolledValue {d3 : 1, d6 : 0, add : 3}),
                tp: RangeWeaponType::Heavy,
                shoot_count: WahValue::Fixed(2),
                strength: 9,
                armor_piercing: -3,
                name: "Redemption lascannons".to_string()
            }),
            FortressOfRedemption::get_krakstorm_missle()
        ];
        Self {
            health : 30,
            weapons,
            name : "Fortress of Redemption".to_string(),
            meele_weapons : vec![]
        }
    }
}

impl Unit for FortressOfRedemption {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_ballistic_skill(&self) -> Option<i32> {
        if self.health >= 16 {
            Some(4)
        } else if self.health >= 9 {
            Some(5)
        } else {
            Some(6)
        }
    }

    fn get_weapon_skill(&self) -> Option<i32> {
        None
    }

    fn get_save(&self) -> i32 {
        3
    }

    fn get_strength(&self) -> i32 {
        6
    }

    fn get_toughness(&self) -> i32 {
        8
    }

    fn get_ranged_weapons(&self) -> &Vec<Box<dyn RangeWeapon>> {
        &self.weapons
    }

    fn get_involve(&self) -> i32 {
        7
    }

    fn add_damage(&mut self, damage : i32, s : i32) -> i32{
        self.health -= damage;
        damage
    }

    fn next_move(&mut self) {
        
    }

    fn set_phase(&mut self, phase : &crate::unit::GamePhase) {
        
    }

    fn get_ignore_ap(&self) -> i32 {
        0
    }

    fn get_meele_attack_count(&self) -> WahValue {
        WahValue::Fixed(0)
    }

    fn is_in_meele_battle(&self) -> bool {
        false
    }

    fn get_meele_weapons(&self) -> &Vec<Box<dyn crate::weapon::MeeleWeapon>> {
        &self.meele_weapons
    }
}