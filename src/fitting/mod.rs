use super::{simulate, DataPointFrame, JW_EMERGENCES};

/// Struct for the data required to fit a required egg coefficient to expected population growth
#[derive(Debug)]
pub struct FittingData {
    delay: f64,
    target_rm: f64,
    gen_time: f64,
    seed_coefficient: f64,
}
impl FittingData {
    /// Create new FittingData from a degree day reference
    pub fn new(delay: f64, target_rm: f64, gen_time: f64, seed_coefficient: f64) -> Self {
        Self {
            delay,
            target_rm,
            gen_time,
            seed_coefficient,
        }
    }

    /// Create new FittingData from a calendar day reference
    pub fn new_from_calendar_day(
        delay: f64,
        gen_time: f64,
        seed_coefficient: f64,
        dd_per_day: f64,
    ) -> Self {
        Self {
            delay,
            target_rm: f64::ln(seed_coefficient) / (dd_per_day * gen_time),
            gen_time: gen_time * dd_per_day,
            seed_coefficient,
        }
    }

    /// Create an array of [FittingData]s from arrays of it's consitutent fields
    pub fn new_many<const N: usize>(
        delay: [f64; N],
        target_rm: [f64; N],
        gen_time: [f64; N],
        seed_coefficient: [f64; N],
    ) -> [Self; N] {
        (0..N)
            .into_iter()
            .map(|n| Self::new(delay[n], target_rm[n], gen_time[n], seed_coefficient[n]))
            .collect::<Vec<Self>>()
            .try_into()
            .expect("Fixed size map should have exactly N elements")
    }

    /// Same as [Self::new_many] but with a calendar day reference
    pub fn new_many_from_calendar_day<const N: usize>(
        delay: [f64; N],
        gen_time: [f64; N],
        seed_coefficient: [f64; N],
        dd_per_day: f64,
    ) -> [Self; N] {
        (0..N)
            .into_iter()
            .map(|n| {
                Self::new_from_calendar_day(delay[n], gen_time[n], seed_coefficient[n], dd_per_day)
            })
            .collect::<Vec<Self>>()
            .try_into()
            .expect("Fixed size map should have exactly N elements")
    }
}

/// Finds the egg_coefficients that would be required to match the population growth data supplied to it
pub fn fit_pop_growth<const N: usize>(fitparams: [FittingData; N]) -> [f64; N] {
    (0..N)
        .into_iter()
        .map(|n| {
            let mut coefficient = fitparams[n].seed_coefficient;
            let mut rm = 1.0;
            while f64::abs(1.0 - (rm / fitparams[n].target_rm)) > 0.001 {
                let data = simulate(
                    100_000,
                    0.05,
                    0..=2200,
                    JW_EMERGENCES,
                    fitparams[n].delay,
                    coefficient,
                );
                rm = calc_inherent_growth(
                    100_000.0,
                    get_last_egg_total(&data, 2),
                    fitparams[n].gen_time * 2.0,
                );
                coefficient *= 1.0 / (rm / fitparams[n].target_rm);
            }
            coefficient
        })
        .collect::<Vec<f64>>()
        .try_into()
        .expect("Fixed size map should have exactly N elements")
}

pub fn calc_inherent_growth(n0: f64, nn: f64, t: f64) -> f64 {
    f64::ln(nn / n0) / t
}

pub fn get_last_egg_total<const NUM_GEN: usize>(data: &DataPointFrame<NUM_GEN>, gen: usize) -> f64 {
    data.0
        .get(2200)
        .expect("Couldn't get 2200th datapoint")
        .eggs_total
        .get(gen + 1)
        .expect("Couldn't get {gen} egg_total")
        .clone()
}
