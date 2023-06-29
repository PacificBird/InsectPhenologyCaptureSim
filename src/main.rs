use std::io::Write;
use InsectPhenologyCaptureSim::{
    egg_coefficient,
    multisim::{multisim, MultiParam},
    // fitting::{fit_pop_growth, FittingData},
    simulate,
    JW_EMERGENCES,
};

fn main() {
    // println!(
    //     "fitted vals: {:#?}",
    //     fit_pop_growth(FittingData::new_many_from_calendar_day(
    //         [0.0, 0.0, 24.0, 40.0, 48.0, 50.0, 72.0],
    //         [55.2, 53.7, 47.77, 57.3, 37.88, 66.5],
    //         [15.0, 13.1, 9.7, 9.0, 5.5, 8.6],
    //         12.0,
    //     ))
    // );

    let mating_delay = 0.0;
    let test_0 = simulate(
        100_000,
        0.00025,
        0..=2200,
        JW_EMERGENCES,
        mating_delay,
        egg_coefficient(mating_delay),
    );
    let test_0_csv = test_0.to_csv_string();
    let mut file_0 = std::fs::File::create("test_0.csv").unwrap();
    writeln!(&mut file_0, "{}", test_0_csv).expect("Couldn't write test_0.csv");

    let test_1 = multisim(
        MultiParam::Range(100.0..=100_100.0, 10),
        // MultiParam::Constant(100_000.0),
        MultiParam::Range(0.001..=0.101, 1),
        // MultiParam::Constant(0.05),
        0..=2200,
        JW_EMERGENCES,
        // MultiParam::Range(0.0..=80.0, 4),
        MultiParam::Constant(0.0),
        egg_coefficient,
    );
    println!("after simulation, size is {}", test_1.0.len());
    let test_1_csv = test_1.to_csv_string();
    println!("before file creation");
    let mut file_1 = std::fs::File::create("test_1.csv").unwrap();
    println!("after file creation");
    writeln!(&mut file_1, "{}", test_1_csv).expect("Couldn't write test_0.csv");
    println!("after file write");
}
