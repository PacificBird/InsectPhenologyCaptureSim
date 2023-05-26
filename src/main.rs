use rand::prelude::*;
use std::{
    f64::consts::{E, PI},
    fmt,
    fmt::{Display, Formatter},
    io::Write,
};

struct DataPoint {
    pop_captured: u32,
    pop_0: f64,
    pop_active: u32,
    eggs: u32,
}
impl Display for DataPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.pop_captured, self.pop_0, self.pop_active, self.eggs
        )
    }
}

#[allow(dead_code)]
enum ProbDist<'a> {
    CDF(&'a dyn Fn(f64) -> f64),
    PDF(&'a dyn Fn(f64) -> f64),
}

fn main() {
    // let pop_range = 1_000..=100_000;
    // let delta_pop = 100;
    // let detection_range = 0.0025..=0.050;
    // let delta_detection = 0.0005;
    let degree_day_range = 0..=1000;

    // println!("{}", fit_egg_value(0.0, 15.0));
    // println!("{}", fit_egg_value(40.0, 9.7));
    // println!("{}", fit_egg_value(50.0, 8.6));

    let mating_delay = 50.0;
    let test_0 = simulate(
        100_000,
        0.00025,
        degree_day_range.clone(),
        ProbDist::PDF(&jones_wiman_2012),
        mating_delay,
        egg_coefficient(mating_delay),
    );
    // println!("{}", egg_coefficient(mating_delay));
    let test_0_csv = test_0
        .into_iter()
        .enumerate()
        .map(|(idx, x)| format!("{},{}\n", idx, x.to_string()))
        .reduce(|acc, row| format!("{}{}", acc, row))
        .unwrap();
    let test_0_csv = format!("dd,captured,population,active,eggs\n{}", test_0_csv);
    let mut file_0 = std::fs::File::create("test_0.csv").unwrap();
    writeln!(&mut file_0, "{}", test_0_csv).expect("Couldn't write test_0.csv");
}

fn diff(x: f64, cdf: impl Fn(f64) -> f64) -> f64 {
    let h = 0.001;
    (cdf(x + h) - cdf(x)) / h
}

fn simulate(
    pop_0: u32,
    prob_detection: f64,
    deg_day_range: std::ops::RangeInclusive<u32>,
    emergence: ProbDist,
    mating_delay: f64,
    egg_multiplier: f64,
) -> Vec<DataPoint> {
    let round_correction = 1.5805019683;
    let mut pop_inactive = pop_0 as f64;
    pop_inactive *= round_correction;
    let mut pop_active = 0;
    let mut pop_captured = 0;
    let mut avg_age: f64 = 0.0;
    let mut eggs: u32 = 0;
    let mut pop_active_last = 0;

    let mating_chance = 0.45;

    deg_day_range
        .into_iter()
        .map(|x| {
            let activated = ((pop_inactive
                * match emergence {
                    ProbDist::CDF(cdf) => diff(x as f64, &*cdf),
                    ProbDist::PDF(pdf) => pdf(x as f64),
                })
                / 100.0)
                .max(0.0);
            pop_inactive -= activated;
            pop_active += activated as u32;
            // mating_pop += (activated / 2.0) as u32;

            avg_age += 1.0
                * if pop_active != 0 {
                    pop_active_last as f64 / pop_active as f64
                } else {
                    0.0
                };

            if pop_active == 0 {
                avg_age *= 0.0;
            }

            let mut rng = thread_rng();
            let (captured, died): (u32, u32) =
                (0..pop_active).fold((0u32, 0u32), |(cap, ded), _| {
                    if rng.gen::<f64>() <= prob_detection {
                        (cap + 1, ded)
                    } else if rng.gen::<f64>()
                        >= E.powf(0.059 * (1f64 - E.powf(0.044 * avg_age as f64)))
                    {
                        (cap, ded + 1)
                    } else {
                        (cap, ded)
                    }
                });

            let delay_steepness = 3.0;
            let mating_now = ((pop_active as f64 / 2.0)
                * mating_chance
                * adjusted_logistic(delay_steepness, mating_delay, avg_age))
                as u32;

            eggs += (egg_multiplier * mating_now as f64) as u32;

            // println!("{},{},{},{}", pop_active, captured, died, avg_age);
            pop_active -= captured;
            pop_captured += captured;
            pop_active = pop_active.checked_sub(died).unwrap_or(0);

            // mating delay debug stuff

            // if x % 5 == 0 {
            //     println!(
            //         "{x}, {pop_active}, mating_now: {mating_now}, age: {avg_age}, proprtion: {}",
            //         pop_active_last as f64 / pop_active as f64
            //     );
            // }

            pop_active_last = pop_active;
            // avg_age *= ((pop_active != 0) as i32) as f64;

            DataPoint {
                pop_captured,
                pop_0: ((round_correction * pop_0 as f64) - pop_inactive),
                pop_active,
                eggs,
            }
        })
        .collect::<Vec<DataPoint>>()
}

#[allow(dead_code)]
fn example_cdf(x: f64) -> f64 {
    100.0 / (1.0 + f64::exp(-0.05 * (x - 200.0)))
}

fn jones_wiman_2012(x: f64) -> f64 {
    let gamma_0 = 1.0737;
    let delta_0 = 1.2349;
    let lambda_0 = 577.2;
    let zeta_0 = 69.0;
    let z_0 = (x - zeta_0) / lambda_0;

    let gamma_1 = 0.3964;
    let delta_1 = 1.4682;
    let lambda_1 = 825.6;
    let zeta_1 = 494.8;
    let z_1 = (x - zeta_1) / lambda_1;

    let gamma_2 = 0.0876;
    let delta_2 = 1.0923;
    let lambda_2 = 746.9;
    let zeta_2 = 1101.2;
    let z_2 = (x - zeta_2) / lambda_2;

    (100.0
        * (delta_0 / (lambda_0 * (2.0 * PI).sqrt() * z_0 * (1.0 - z_0)))
        * f64::exp(-0.5 * (gamma_0 + (delta_0 * (z_0 / (1.0 - z_0)).ln())).powi(2)))
    .max(0.0) + 
    (100.0
        * (delta_1 / (lambda_1 * (2.0 * PI).sqrt() * z_1 * (1.0 - z_1)))
        * f64::exp(-0.5 * (gamma_1 + (delta_1 * (z_1 / (1.0 - z_1)).ln())).powi(2)))
    .max(0.0) + 
    (100.0
        * (delta_2 / (lambda_2 * (2.0 * PI).sqrt() * z_2 * (1.0 - z_2)))
        * f64::exp(-0.5 * (gamma_2 + (delta_2 * (z_2 / (1.0 - z_2)).ln())).powi(2)))
    .max(0.0)
}

// fn normal(mean: f64, stdev: f64, x: f64) -> f64 {
//     (1.0 / (stdev * (2.0 * PI).sqrt())) * E.powf(-0.5 * ((x - mean) / stdev).powi(2))
// }

// fn curried_normal(mean: f64, stdev: f64) -> impl Fn(f64) -> f64 {
//     move |x| normal(mean, stdev, x)
// }

fn adjusted_logistic(steepness: f64, translation: f64, x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-1.0 * steepness * (x as f64 - translation + (2.2 / steepness))))
}

#[allow(dead_code)]
fn fit_egg_value(delay: f64, target: f64) -> f64 {
    let mut fit = 1.0;
    let mut net_repr_rate = 0.0;
    while (net_repr_rate - target).abs() > 0.01 {
        let result = simulate(
            100_000,
            0.0,
            0..=1000,
            ProbDist::PDF(&jones_wiman_2012),
            delay,
            fit,
        )[1000]
            .eggs
            .clone();

        let eggs = result as f64;
        net_repr_rate = eggs / 100_000.0;
        println!("fit: {fit}, eggs: {eggs}, repr_rate: {net_repr_rate}");

        fit *= target / net_repr_rate;
    }

    fit
}

fn egg_coefficient(delay: f64) -> f64 {
    (0.005147 * delay * delay) - (0.3227 * delay) + 36.02
}
