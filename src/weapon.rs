
use crate::unit::Unit;
use crate::{Game, Roller, WahValue};
use log::*;

#[derive(Copy, Clone, Debug)]
pub enum RangeWeaponType {
    Assault,
    Heavy
}

pub trait BaseWeapon {
    fn get_strength(&self) -> i32;
}

pub trait RangeWeapon : BaseWeapon {
    fn set_target(&mut self, unit : &dyn Unit);
    fn get_damage(&self) -> WahValue;
    fn get_type(&self) -> RangeWeaponType;
    fn get_attack_count(&self) -> WahValue;
    fn get_armor_piercing(&self) -> i32;
    fn get_name(&self) -> &String;

    fn get_damage_to(&self, unit : &dyn Unit, target : &dyn Unit, roller : &mut Roller) -> (WahValue, i32) {
        //ballistic skill check
        let bs = unit.get_ballistic_skill().unwrap();
        let attack_count = self.get_attack_count().get(roller);
        //BS check
        let mut bs_cheched = roller.d6check(attack_count, bs);
        // info!("[{}][{}] Ballistic checked {} from {}",unit.get_name(), self.get_name(), bs_cheched, attack_count);
        //T check
        let mut toughness_checked = roller.d6check(
            bs_cheched,
            Game::get_toughness_check(
                self.get_strength(),
                target.get_toughness()
            ));
        // info!("[{}][{}] Toughness checked {} from {}",unit.get_name(), self.get_name(), toughness_checked, bs_cheched);

        //save check
        let ap_ignore = {
            if self.get_armor_piercing() < 0 {
                target.get_ignore_ap()
            } else {
                0
            }
        };
        let save_check = std::cmp::min(target.get_save() - self.get_armor_piercing() - ap_ignore, target.get_involve());
        let save_checked_count = toughness_checked - roller.d6check(toughness_checked, save_check);
        // info!("[{}][{}] Save checked {} from {}",unit.get_name(), self.get_name(), save_checked_count, toughness_checked);

        let single_damage = self.get_damage();
        (single_damage, save_checked_count)
    }
}

pub struct StandardRangeWeapon {
    pub damage : WahValue,
    pub tp : RangeWeaponType,
    pub shoot_count : WahValue,
    pub strength : i32,
    pub armor_piercing : i32,
    pub name : String
}

impl BaseWeapon for StandardRangeWeapon {
    fn get_strength(&self) -> i32 {
        self.strength
    }
}

impl RangeWeapon for StandardRangeWeapon {
    fn set_target(&mut self, unit: &dyn Unit) {

    }

    fn get_damage(&self) -> WahValue {
        self.damage
    }

    fn get_type(&self) -> RangeWeaponType {
        self.tp
    }

    fn get_attack_count(&self) -> WahValue {
        self.shoot_count
    }

    fn get_armor_piercing(&self) -> i32 {
        self.armor_piercing
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct StandardMeeleWeapon {
    pub damage : WahValue,
    pub attack_mul : i32,
    pub strength : i32,
    pub armor_piercing : i32,
    pub name : String
}
pub trait MeeleWeapon : BaseWeapon {
    fn set_target(&mut self, unit : &dyn Unit);
    fn get_damage(&self) -> WahValue;
    fn get_attack_mul(&self) -> i32;
    fn get_armor_piercing(&self) -> i32;
    fn get_name(&self) -> &String;

    fn get_damage_to(&self, unit : &dyn Unit, target : &dyn Unit, roller : &mut Roller) -> (WahValue, i32) {
        //ballistic skill check
        let ws = unit.get_weapon_skill().unwrap();
        let attack_count = unit.get_meele_attack_count().get(roller) * self.get_attack_mul();
        //BS check
        let ws_cheched = roller.d6check(attack_count, ws);
        // info!("[{}][{}] Ballistic checked {} from {}",unit.get_name(), self.get_name(), bs_cheched, attack_count);
        //T check
        let toughness_checked = roller.d6check(
            ws_cheched,
            Game::get_toughness_check(
                self.get_strength(),
                target.get_toughness()
            ));
        // info!("[{}][{}] Toughness checked {} from {}",unit.get_name(), self.get_name(), toughness_checked, bs_cheched);

        //save check
        let ap_ignore = {
            if self.get_armor_piercing() < 0 {
                target.get_ignore_ap()
            } else {
                0
            }
        };
        let save_check = std::cmp::min(target.get_save() - self.get_armor_piercing() - ap_ignore, target.get_involve());
        let save_checked_count = toughness_checked - roller.d6check(toughness_checked, save_check);
        // info!("[{}][{}] Save checked {} from {}",unit.get_name(), self.get_name(), save_checked_count, toughness_checked);

        let single_damage = self.get_damage();
        (single_damage, save_checked_count)
    }

}

impl BaseWeapon for StandardMeeleWeapon {
    fn get_strength(&self) -> i32 {
        self.strength
    }
}

impl MeeleWeapon for StandardMeeleWeapon {
    fn set_target(&mut self, unit : &dyn Unit) {
        
    }

    fn get_damage(&self) -> WahValue {
        self.damage
    }

    fn get_attack_mul(&self) -> i32 {
        self.attack_mul
    }

    fn get_armor_piercing(&self) -> i32 {
        self.armor_piercing
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}