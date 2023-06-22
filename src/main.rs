use std::io::Write;
use InsectPhenologyCaptureSim::{
    egg_coefficient,
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
    let test_0_headers = test_0.0.get(1).unwrap().csv_headers();
    let test_0_csv = test_0
        .0
        .into_iter()
        .enumerate()
        .map(|(idx, x)| format!("{},{}\n", idx, x.to_string()))
        .reduce(|acc, row| format!("{}{}", acc, row))
        .unwrap();
    let test_0_csv = format!("{}\n{}", test_0_headers, test_0_csv);
    let mut file_0 = std::fs::File::create("test_0.csv").unwrap();
    writeln!(&mut file_0, "{}", test_0_csv).expect("Couldn't write test_0.csv");
}
