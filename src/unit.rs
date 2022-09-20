use log::warn;
use crate::{Roller, WahValue};
use crate::weapon::{RangeWeapon, MeeleWeapon, BaseWeapon};
use log::*;

pub enum GamePhase {
    Meele,
    Shoot
}

pub trait Unit {
    fn get_name(&self) -> &String;
    fn get_health(&self) -> i32;
    fn get_ballistic_skill(&self) -> Option<i32>;
    fn get_weapon_skill(&self) -> Option<i32>;
    fn get_save(&self) -> i32;
    fn get_strength(&self) -> i32;
    fn get_toughness(&self) -> i32;
    fn get_ranged_weapons(&self) -> &Vec<Box<dyn RangeWeapon>>;
    fn get_involve(&self) -> i32;
    fn add_damage(&mut self, damage : i32, s : i32) -> i32;
    fn next_move(&mut self);
    fn set_phase(&mut self, phase : &GamePhase);
    fn get_ignore_ap(&self) -> i32;
    fn get_meele_attack_count(&self) -> WahValue;
    fn is_in_meele_battle(&self) -> bool;
    fn get_meele_weapons(&self) -> &Vec<Box<dyn MeeleWeapon>>;

    fn pretty_report_ballistic_skill(&self, roller : &mut Roller) {
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
}




//
// pub struct GroundUnit {
//     health : i32,
//     bullet_skill : Option<i32>,
//     weapon_skill : Option<i32>,
//     save : i32,
//     strength : i32,
//     toughness : i32,
//     range_weapons: Vec<Box<dyn RangeWeapon>>
// }
//
// impl Unit for GroundUnit {
//     fn get_name(&self) -> &String {
//
//     }
//
//     fn get_health(&self) -> i32 {
//         self.health
//     }
//
//     fn get_ballistic_skill(&self) -> Option<i32> {
//         self.bullet_skill
//     }
//
//     fn get_weapon_skill(&self) -> Option<i32> {
//         self.weapon_skill
//     }
//
//     fn get_save(&self) -> i32 {
//         self.save
//     }
//
//     fn get_strength(&self) -> i32 {
//         self.strength
//     }
//
//     fn get_toughness(&self) -> i32 {
//         self.toughness
//     }
//
//     fn get_ranged_weapons(&self) -> &Vec<Box<dyn RangeWeapon>> {
//         &self.range_weapons
//     }
//
//     fn get_involve(&self) -> i32 {
//         
//     }
// }