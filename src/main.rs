use std::fs::File;
use log::{info, LevelFilter};
use rand::Rng;
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};
use warhammer_roller::*;
use warhammer_roller::unit::Unit;
use warhammer_roller::units::FortressOfRedemption;
use log::*;


fn main() {

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    let mut roller = Roller::default();


    let mut sum_moves = 0;
    let stat_size = 1000;

    for try_idx in 0..stat_size {
        let mut moves = 0;
        let mut fortress1 = FortressOfRedemption::default();
        let mut fortress2 = FortressOfRedemption::default();

        while fortress1.get_health() > 0 && fortress2.get_health() > 0 {
            Game::full_range_attack_to(
                &mut fortress1,
                &mut fortress2,
                &mut roller
            );
            Game::full_range_attack_to(
                &mut fortress2,
                &mut fortress1,
                &mut roller
            );
            moves += 1;
        }
        sum_moves += moves as u64;
    }



    warn!("Moves: {}",
        (sum_moves as f32) / (stat_size as f32));

}
