
use crate::unit::Unit;
use crate::{Game, Roller, WahValue};
use log::*;

#[derive(Copy, Clone, Debug)]
pub enum RangeWeaponType {
    Assault,
    Heavy
}

pub trait RangeWeapon {
    fn set_target(&mut self, unit : &dyn Unit);
    fn get_damage(&self) -> WahValue;
    fn get_type(&self) -> RangeWeaponType;
    fn get_attack_count(&self) -> WahValue;
    fn get_strength(&self) -> i32;
    fn get_armor_piercing(&self) -> i32;
    fn get_name(&self) -> &String;

    fn get_damage_to(&self, unit : &dyn Unit, target : &dyn Unit, roller : &mut Roller) -> i32 {
        //ballistic skill check
        let bs = unit.get_ballistic_skill().unwrap();
        let attack_count = self.get_attack_count().get(roller);
        //BS check
        let mut bs_cheched = roller.d6check(attack_count, bs);
        info!("[{}][{}] Ballistic checked {} from {}",unit.get_name(), self.get_name(), bs_cheched, attack_count);
        //T check
        let mut toughness_checked = roller.d6check(
            bs_cheched,
            Game::get_toughness_check(
                self.get_strength(),
                target.get_toughness()
            ));
        info!("[{}][{}] Toughness checked {} from {}",unit.get_name(), self.get_name(), toughness_checked, bs_cheched);

        //save check
        let save_check = target.get_save() + self.get_armor_piercing();
        let save_checked_count;
        if save_check < target.get_involve() {
            save_checked_count = toughness_checked - roller.d6check(toughness_checked, save_check);
            info!("[{}][{}] Save checked {} from {}",unit.get_name(), self.get_name(), save_checked_count, toughness_checked);
        } else {
            save_checked_count = toughness_checked - roller.d6check(toughness_checked, target.get_involve());
            info!("[{}][{}] Involve checked {} from {}",unit.get_name(), self.get_name(), save_checked_count, toughness_checked);
        }


        //Calculate damage
        let mut damage = 0;
        let single_damage = self.get_damage();
        for i in 0..save_checked_count {
            damage += single_damage.get(roller);
        }
        info!("[{}][{}] Damage done {}",unit.get_name(), self.get_name(), damage);

        damage
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

    fn get_strength(&self) -> i32 {
        self.strength
    }

    fn get_armor_piercing(&self) -> i32 {
        self.armor_piercing
    }

    fn get_name(&self) -> &String {
        &self.name
    }


}