use std::{
    f64::consts::PI,
    fmt,
    fmt::{Display, Formatter},
    io::Write,
};

struct DataPoint {
    pop_captured: f64,
    pop_active: f64,
    eggs: f64,
}
impl Display for DataPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.pop_captured, self.pop_active, self.eggs,)
    }
}

#[allow(dead_code)]
enum ProbDist<'a> {
    CDF(&'a dyn Fn(f64) -> f64),
    PDF(&'a dyn Fn(f64) -> f64),
}

fn main() {
    // println!("{}", fit_egg_value(0.0, 15.0));
    // println!("{}", fit_egg_value(40.0, 9.7));
    // println!("{}", fit_egg_value(50.0, 8.6));

    let mating_delay = 50.0;
    let test_0 = simulate(
        100_000,
        0.00025,
        0..=2200,
        vec![
            ProbDist::PDF(&jones_wiman_2012_0),
            ProbDist::PDF(&jones_wiman_2012_1),
            ProbDist::PDF(&jones_wiman_2012_2),
        ],
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
    let test_0_csv = format!("dd,captured,population_active,eggs\n{}", test_0_csv);
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
    emergences: Vec<ProbDist>,
    mating_delay: f64,
    egg_multiplier: f64,
) -> Vec<DataPoint> {
    let num_generations = emergences.len();
    let mut pop_active: Vec<f64> = (0..num_generations).into_iter().map(|_| 0.0).collect();
    let mut pop_captured = 0.0;
    let mut avg_age: Vec<f64> = (0..num_generations).into_iter().map(|_| 0.0).collect();
    let mut eggs: Vec<f64> = (0..num_generations + 1).into_iter().map(|_| 0.0).collect();
    eggs[0] = pop_0 as f64;
    let mut eggs_total: Vec<f64> = (0..num_generations + 1).into_iter().map(|_| 0.0).collect();
    eggs_total[0] = pop_0 as f64;
    let mut pop_active_last: Vec<f64> = (0..num_generations).into_iter().map(|_| 0.0).collect();

    let mating_chance = 0.45;

    deg_day_range
        .into_iter()
        .map(|x| {
            for generation in (0..num_generations).into_iter() {
                let activated = (eggs_total[generation]
                    * match emergences[generation] {
                        ProbDist::CDF(cdf) => diff(x as f64, &*cdf),
                        ProbDist::PDF(pdf) => pdf(x as f64),
                    })
                .max(0.0);
                eggs[generation] -= activated;
                pop_active[generation] += activated;
                avg_age[generation] += 1.0
                    * if pop_active[generation] != 0.0 {
                        pop_active_last[generation] / pop_active[generation]
                    } else {
                        0.0
                    };
                let delay_steepness = 3.0;
                let mating_now = (pop_active[generation] / 2.0)
                    * mating_chance
                    * adjusted_logistic(delay_steepness, mating_delay, avg_age[generation]);

                eggs[generation + 1] += egg_multiplier * mating_now;
                eggs_total[generation + 1] += egg_multiplier * mating_now;

                pop_captured += pop_active[generation] * prob_detection;
                pop_active[generation] = pop_active[generation]
                    * (1.0 - prob_detection)
                    * (f64::exp(0.058 * (1.0 - f64::exp(0.0448 * avg_age[generation]))));
                pop_active_last[generation] = pop_active[generation];
            }

            DataPoint {
                pop_captured,
                pop_active: pop_active.iter().sum(),
                eggs: eggs.iter().sum(),
            }

            // let activated = ((pop_0 as f64
            //     * match emergence {
            //         ProbDist::CDF(cdf) => diff(x as f64, &*cdf),
            //         ProbDist::PDF(pdf) => pdf(x as f64),
            //     })
            //     / 100.0)
            //     .max(0.0);
            // pop_inactive -= activated;
            // pop_active_0 += activated;

            // avg_age_0 += 1.0
            //     * if pop_active_0 != 0.0 {
            //         pop_active_last_0 / pop_active_0
            //     } else {
            //         0.0
            //     };

            // let delay_steepness = 3.0;
            // let mating_now_0 = (pop_active_0 / 2.0)
            //     * mating_chance
            //     * adjusted_logistic(delay_steepness, mating_delay, avg_age_0);

            // eggs_1 += egg_multiplier * mating_now_0 as f64;
            // eggs_1_total += egg_multiplier * mating_now_0 as f64;

            // pop_captured += pop_active_0 * prob_detection;
            // pop_active_0 = pop_active_0
            //     * (1.0 - prob_detection)
            //     * (f64::exp(0.058 * (1.0 - f64::exp(0.0448 * avg_age_0))));

            // // MATING DELAY DEBUG STUFF
            // // if x % 5 == 0 {
            // //     println!(
            // //         "{x}, {pop_active_0}, mating_now: {mating_now_0}, age: {avg_age_0}, proprtion: {}",
            // //         pop_active_last_0 / pop_active_0
            // //     );
            // // }

            // pop_active_last_0 = pop_active_0;

            // let hatched_1 = ((eggs_1_total as f64
            //     * match generation_1 {
            //         ProbDist::CDF(cdf) => diff(x as f64, &*cdf),
            //         ProbDist::PDF(pdf) => pdf(x as f64),
            //     })
            //     / 100.0)
            //     .max(0.0);
            // eggs_1 -= hatched_1;
            // pop_active_1 += hatched_1;

            // avg_age_1 += 1.0
            //     * if pop_active_1 != 0.0 {
            //         pop_active_last_1 as f64 / pop_active_1 as f64
            //     } else {
            //         0.0
            //     };

            // let mating_now_1 = (pop_active_1 as f64 / 2.0) * mating_chance;

            // eggs_2 += egg_coefficient(0.0) * mating_now_1;
            // eggs_2_total += egg_coefficient(0.0) * mating_now_1;

            // pop_active_1 = pop_active_1
            //     * (1.0 - prob_detection)
            //     * (f64::exp(0.059 * (1.0 - f64::exp(0.044 * avg_age_1))));

            // pop_active_last_1 = pop_active_1;

            // let hatched_2 = ((eggs_2_total as f64
            //     * match generation_2 {
            //         ProbDist::CDF(cdf) => diff(x as f64, &*cdf),
            //         ProbDist::PDF(pdf) => pdf(x as f64),
            //     })
            //     / 100.0)
            //     .max(0.0);
            // eggs_2 -= hatched_2;
            // pop_active_2 += hatched_2;

            // avg_age_2 += 1.0
            //     * if pop_active_2 != 0.0 {
            //         pop_active_last_2 as f64 / pop_active_2 as f64
            //     } else {
            //         0.0
            //     };

            // // let mating_now_2 = (pop_active_2 as f64 / 2.0) * mating_chance;

            // // eggs_2 += egg_coefficient(0.0) * mating_now_1;
            // // eggs_2_total += egg_coefficient(0.0) * mating_now_1;

            // pop_active_2 = pop_active_2
            //     * (1.0 - prob_detection)
            //     * (f64::exp(0.059 * (1.0 - f64::exp(0.044 * avg_age_2))));

            // pop_active_last_2 = pop_active_2;

            // DataPoint {
            //     pop_captured,
            //     pop_0: pop_0 as f64 - pop_inactive,
            //     pop_active_0,
            //     pop_active_1,
            //     pop_active_2,
            //     eggs_1,
            //     eggs_2,
            // }
        })
        .collect::<Vec<DataPoint>>()
}

#[allow(dead_code)]
fn example_cdf(x: f64) -> f64 {
    100.0 / (1.0 + f64::exp(-0.05 * (x - 200.0)))
}

fn jones_wiman_2012_0(x: f64) -> f64 {
    let gamma_0 = 1.0737;
    let delta_0 = 1.2349;
    let lambda_0 = 577.2;
    let zeta_0 = 69.0;
    let z_0 = (x - zeta_0) / lambda_0;
    (100.0
        * (delta_0 / (lambda_0 * (2.0 * PI).sqrt() * z_0 * (1.0 - z_0)))
        * f64::exp(-0.5 * (gamma_0 + (delta_0 * (z_0 / (1.0 - z_0)).ln())).powi(2)))
    .max(0.0)
}
fn jones_wiman_2012_1(x: f64) -> f64 {
    let gamma_1 = 0.3964;
    let delta_1 = 1.4682;
    let lambda_1 = 825.6;
    let zeta_1 = 494.8;
    let z_1 = (x - zeta_1) / lambda_1;

    (100.0
        * (delta_1 / (lambda_1 * (2.0 * PI).sqrt() * z_1 * (1.0 - z_1)))
        * f64::exp(-0.5 * (gamma_1 + (delta_1 * (z_1 / (1.0 - z_1)).ln())).powi(2)))
    .max(0.0)
}
fn jones_wiman_2012_2(x: f64) -> f64 {
    let gamma_2 = 0.0876;
    let delta_2 = 1.0923;
    let lambda_2 = 746.9;
    let zeta_2 = 1101.2;
    let z_2 = (x - zeta_2) / lambda_2;

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
            vec![
                ProbDist::PDF(&jones_wiman_2012_0),
                ProbDist::PDF(&jones_wiman_2012_1),
                ProbDist::PDF(&jones_wiman_2012_2),
            ],
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
