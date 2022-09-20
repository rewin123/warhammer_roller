use rand::Rng;
use crate::unit::Unit;

pub mod unit;
pub mod attack_step;
pub mod units;
pub mod weapon;

use log::*;

pub struct Game {}

impl Game {
    fn get_toughness_check(S : i32, T : i32) -> i32 {
        if S >= 2 * T {
            2
        } else if S > T {
            3
        } else if S == T {
            4
        } else if S < T {
            5
        } else if S < 2 * T {
            6
        } else {
            panic!("Unexcepted toughness check");
        }
    }

    pub fn full_range_attack_to(
            target : &mut dyn Unit,
            unit : &mut dyn Unit,
            roller : &mut Roller) {
        let weapons = unit.get_ranged_weapons();
        let mut sum_damage = 0;
        for w in weapons {
            let damage = w.get_damage_to(unit, target, roller);
            sum_damage += target.add_damage(damage);
        }
        warn!("[{}] Make {} damage to [{}]", unit.get_name(), sum_damage, target.get_name());
    }
}

impl Game {
    
}

pub struct Roller {
    rng : rand::prelude::ThreadRng,
    dist6 : rand::distributions::Uniform<i32>,
    dist3 : rand::distributions::Uniform<i32>
}

impl Default for Roller {
    fn default() -> Self {
        Self {
            rng : rand::thread_rng(),
            dist6 : rand::distributions::Uniform::new_inclusive(1, 6),
            dist3 : rand::distributions::Uniform::new_inclusive(1, 3)
        }
    }
}

impl Roller {
    pub fn d6(&mut self) -> i32 {
        self.rng.sample(self.dist6)
    }
    pub fn d3(&mut self) -> i32 {
        self.rng.sample(self.dist3)
    }

    pub fn nd6(&mut self, count : u32) -> i32 {
        let mut res = 0i32;
        for i in 0..count {
            res += self.d6();
        }
        res
    }

    pub fn nd3(&mut self, count : u32) -> i32 {
        let mut res = 0i32;
        for i in 0..count {
            res += self.d3();
        }
        res
    }

    pub fn d6check(&mut self, count : i32, check : i32) -> i32 {
        let mut res = 0;
        for i in 0..count {
            let roll = self.d6();
            if roll >= check {
                res += 1;
            }
        }
        res
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RolledValue {
    d3 : i32,
    d6 : i32,
    add : i32
}

impl RolledValue {
    pub fn roll(&self, roller : &mut Roller) -> i32 {
        roller.d3() * self.d3 + roller.d6() * self.d6 + self.add
    }
}

#[derive(Copy, Clone, Debug)]
pub enum WahValue {
    Fixed(i32),
    Rolled(RolledValue)
}

impl WahValue {
    pub fn get(&self, roller : &mut Roller) -> i32 {
        match &self {
            WahValue::Fixed(val) => {*val}
            WahValue::Rolled(roll) => {roll.roll(roller)}
        }
    }
}
