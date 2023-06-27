#![feature(impl_trait_in_assoc_type)]
use std::f64::consts::PI;

pub mod datapoint;
pub mod fitting;
pub mod multisim;

pub use datapoint::*;

pub fn diff(x: f64, cdf: impl Fn(f64) -> f64) -> f64 {
    let h = 0.001;
    (cdf(x + h) - cdf(x)) / h
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum ProbDist<'a> {
    CDF(&'a dyn Fn(f64) -> f64),
    PDF(&'a dyn Fn(f64) -> f64),
}

pub fn simulate<const NUM_GEN: usize>(
    pop_0: u32,
    prob_detection: f64,
    deg_day_range: std::ops::RangeInclusive<u32>,
    emergences: [ProbDist; NUM_GEN],
    mating_delay: f64,
    egg_multiplier: f64,
) -> DataPointFrame<NUM_GEN> {
    let mut pop_active = [0.0; NUM_GEN];
    let mut pop_captured = 0.0;
    let mut avg_age = [0.0; NUM_GEN];
    let mut eggs = [0.0; NUM_GEN];
    eggs[0] = pop_0 as f64;
    let mut eggs_total = [0.0; NUM_GEN];
    eggs_total[0] = pop_0 as f64;
    let mut pop_active_last = [0.0; NUM_GEN];

    let mating_chance = 0.45;

    DataPointFrame(
        deg_day_range
            .into_iter()
            .map(|x| {
                for generation in (0..NUM_GEN).into_iter() {
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

                    if generation + 1 < NUM_GEN {
                        eggs[generation + 1] += egg_multiplier * mating_now;
                        eggs_total[generation + 1] += egg_multiplier * mating_now;
                    }

                    pop_captured += pop_active[generation] * prob_detection;
                    pop_active[generation] = pop_active[generation]
                        * (1.0 - prob_detection)
                        * (f64::exp(0.058 * (1.0 - f64::exp(0.0448 * avg_age[generation]))));
                    pop_active_last[generation] = pop_active[generation];
                }

                DataPoint {
                    pop_captured,
                    pop_active: pop_active.clone(),
                    eggs: eggs.clone(),
                    eggs_total: eggs_total.clone(),
                }
            })
            .collect::<Vec<DataPoint<NUM_GEN>>>(),
    )
}

#[allow(dead_code)]
pub fn example_cdf(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-0.05 * (x - 200.0)))
}

pub fn jones_wiman_2012_0(x: f64) -> f64 {
    let gamma_0 = 1.0737;
    let delta_0 = 1.2349;
    let lambda_0 = 577.2;
    let zeta_0 = 69.0;
    let z_0 = (x - zeta_0) / lambda_0;
    (1.0 * (delta_0 / (lambda_0 * (2.0 * PI).sqrt() * z_0 * (1.0 - z_0)))
        * f64::exp(-0.5 * (gamma_0 + (delta_0 * (z_0 / (1.0 - z_0)).ln())).powi(2)))
    .max(0.0)
}
pub fn jones_wiman_2012_1(x: f64) -> f64 {
    let gamma_1 = 0.3964;
    let delta_1 = 1.4682;
    let lambda_1 = 825.6;
    let zeta_1 = 494.8;
    let z_1 = (x - zeta_1) / lambda_1;

    (1.0 * (delta_1 / (lambda_1 * (2.0 * PI).sqrt() * z_1 * (1.0 - z_1)))
        * f64::exp(-0.5 * (gamma_1 + (delta_1 * (z_1 / (1.0 - z_1)).ln())).powi(2)))
    .max(0.0)
}
pub fn jones_wiman_2012_2(x: f64) -> f64 {
    let gamma_2 = 0.0876;
    let delta_2 = 1.0923;
    let lambda_2 = 746.9;
    let zeta_2 = 1101.2;
    let z_2 = (x - zeta_2) / lambda_2;

    (1.0 * (delta_2 / (lambda_2 * (2.0 * PI).sqrt() * z_2 * (1.0 - z_2)))
        * f64::exp(-0.5 * (gamma_2 + (delta_2 * (z_2 / (1.0 - z_2)).ln())).powi(2)))
    .max(0.0)
}

pub const JW_EMERGENCES: [ProbDist; 3] = [
    ProbDist::PDF(&jones_wiman_2012_0),
    ProbDist::PDF(&jones_wiman_2012_1),
    ProbDist::PDF(&jones_wiman_2012_2),
];

pub fn adjusted_logistic(steepness: f64, translation: f64, x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-1.0 * steepness * (x as f64 - translation + (2.2 / steepness))))
}

pub fn egg_coefficient(delay: f64) -> f64 {
    (0.005147 * delay * delay) - (0.3227 * delay) + 36.02
}
