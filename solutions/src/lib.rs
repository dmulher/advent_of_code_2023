#![feature(test)]

mod day_01_a;
mod day_01_b;
mod day_02_a;
mod day_02_b;
mod day_03_a;
mod day_03_b;
mod day_04_a;
mod day_04_b;

pub fn run_all_days() {
    day_01_a::get_calibration("".to_string());
    day_01_b::get_calibration("".to_string());
    day_02_a::get_possible_games("".to_string());
    day_02_b::get_game_powers("".to_string());
    day_03_a::sum_active_symbols("".to_string());
    day_03_b::sum_active_symbols("".to_string());
    day_04_a::sum_active_symbols("".to_string());
    day_04_b::sum_active_symbols("".to_string());
}