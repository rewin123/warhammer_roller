use std::fs::File;
use log::{info, LevelFilter};
use rand::Rng;
use simplelog::{ColorChoice, CombinedLogger, Config, TerminalMode, TermLogger, WriteLogger};
use warhammer_roller::*;
use warhammer_roller::unit::{Unit, GamePhase};
use warhammer_roller::units::{FortressOfRedemption, Abaddon};
use log::*;
use plotters::prelude::*;

#[derive(Debug)]
struct UnitStat {
    health : Vec<Vec<i32>>,
    name : String
}

impl Default for UnitStat {
    fn default() -> Self {
        Self { 
            health: vec![],
            name : "".to_string() 
        }
    }
}

impl UnitStat {
    fn push_health(&mut self, val : i32, move_idx : usize) {
        if self.health.len() > move_idx {
            self.health[move_idx].push(val);
        } else {
            while self.health.len() <= move_idx {
                self.health.push(vec![]);
            }
            self.push_health(val, move_idx);
        }
    }

    fn push_data(&mut self, unit : &dyn Unit, move_idx : usize) {
        self.push_health(unit.get_health(), move_idx);
    }

    fn collect_stat(data : &Vec<Vec<i32>>) -> Vec<(f32, f32)> {
        data.iter().map(|point| {
            let mut m_x = 0.0;
            let mut m_x2 = 0.0;
            for val in point {
                m_x += *val as f32;
                m_x2 += (*val as f32) * (*val as f32);
            }
            m_x = m_x / point.len() as f32; 
            m_x2 = m_x2 / point.len() as f32;
            let d = (m_x2 - m_x * m_x).sqrt();

            (m_x, d)
        }).collect()
    }

    fn create_health_plot(path : String, stats : &[UnitStat]) -> Result<(), Box<dyn std::error::Error>> {
        
        let mut datas = vec![];
        for s in stats {
            datas.push(
                UnitStat::collect_stat(&s.health)
            );
        }

        //determine grid
        let mut miv = 0.0;
        let mut mav = 0.0;
        for d in &datas {
            for (v, _) in d {
                if *v < miv {
                    miv = *v;
                }
                if *v > mav {
                    mav = *v;
                }
            }
        }

        let styles = vec![
            BLUE.filled(),
            RED.filled()
        ];
        
        let root = BitMapBackend::new(path.as_str(), (1024, 768)).into_drawing_area();

        root.fill(&WHITE)?;
    
        let mut chart = ChartBuilder::on(&root)
            .caption("Health", ("sans-serif", 60))
            .margin(10)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0f32..(datas[0].len() as f32), (miv as f32)..(mav as f32))?;

        chart.configure_mesh().draw()?;

        for (idx, d) in datas.iter().enumerate() {
            chart.draw_series(
                d.iter().enumerate().map(|(x, (v, dv))| {
                    ErrorBar::new_vertical(x as f32, *v - *dv, *v, *v + *dv, styles[idx], 20)
                }),
            )?;
        }


        root.present()?;

        Ok(())
    }
}

fn main() {

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    let mut roller = Roller::default();

    let mut unit1_stat = UnitStat::default();
    let mut unit2_stat = UnitStat::default();

    unit1_stat.name = "Abbadon".to_string();
    unit2_stat.name = "Fort".to_string();

    let mut sum_moves = 0;
    let mut unit1_wins = 0;
    let mut unit2_wins = 0;
    let stat_size = 1000;



    for try_idx in 0..stat_size {
        let mut moves = 0;
        let mut fortress1 = Abaddon::default();
        let mut fortress2 = FortressOfRedemption::default();

        while fortress1.get_health() > 0 && fortress2.get_health() > 0 {
            fortress1.next_move();
            fortress2.next_move();
            fortress1.set_phase(&GamePhase::Shoot);
            fortress2.set_phase(&GamePhase::Shoot);
            
            Game::full_range_attack_to(
                &mut fortress2,
                &mut fortress1,
                &mut roller
            );
            Game::full_range_attack_to(
                &mut fortress1,
                &mut fortress2,
                &mut roller
            );
            unit1_stat.push_data(&fortress1, moves as usize);
            unit2_stat.push_data(&fortress2 , moves as usize);
            moves += 1;
        }
        sum_moves += moves as u64;
        if fortress1.get_health() == 0 {
            unit2_wins += 1;
        } else {
            unit1_wins += 1;
        }
    }

    warn!("Moves: {}",
        (sum_moves as f32) / (stat_size as f32));
    warn!("Wins: {}:{}",
        unit1_wins, unit2_wins);
        warn!("Wins (%): {}:{} ([{}],[{}])",
            unit1_wins as f32 / stat_size as f32 * 100.0, 
            unit2_wins as f32 / stat_size as f32 * 100.0,
            &unit1_stat.name,
            &unit2_stat.name);

    UnitStat::create_health_plot(
        "health.png".to_string(),
        &[unit1_stat, unit2_stat]).unwrap();

}
