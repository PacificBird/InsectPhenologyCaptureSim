use super::{simulate, JW_EMERGENCES};
use crate::DataPoint;
use std::fmt::{Display, Formatter};

pub struct TaggedDataPoint<const NUM_GEN: usize> {
    data: DataPoint<NUM_GEN>,
    pop_0: u32,
    prob_capture: f64,
    mating_delay: f64,
}

impl<const NUM_GEN: usize> TaggedDataPoint<NUM_GEN> {
    pub fn new(data: DataPoint<NUM_GEN>, pop_0: u32, prob_capture: f64, mating_delay: f64) -> Self {
        Self {
            data,
            pop_0,
            prob_capture,
            mating_delay,
        }
    }

    pub fn new_vec(
        data: Vec<DataPoint<NUM_GEN>>,
        pop_0: u32,
        prob_capture: f64,
        mating_delay: f64,
    ) -> Vec<Self> {
        data.into_iter()
            .map(|datapoint| Self {
                data: datapoint,
                pop_0,
                prob_capture,
                mating_delay,
            })
            .collect()
    }

    pub fn csv_headers(&self) -> String {
        format!(
            "{},pop_0,prob_capture,mating_delay",
            self.data.csv_headers()
        )
    }
}

impl<const NUM_GEN: usize> Display for TaggedDataPoint<NUM_GEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.data.to_string(),
            self.pop_0,
            self.prob_capture,
            self.mating_delay,
        )
    }
}

pub struct TaggedDataPointFrame<const NUM_GEN: usize>([TaggedDataPoint<NUM_GEN>]);

impl<const NUM_GEN: usize> TaggedDataPointFrame<NUM_GEN> {
    // pub fn from_untagged_frame()
    pub fn to_csv_string(&self) -> String {
        let headers = self
            .0
            .get(0)
            .expect("Couldn't get first TaggedDataPoint")
            .csv_headers();
        let data = self
            .0
            .into_iter()
            .enumerate()
            .map(|(idx, x)| format!("{},{}\n", idx, x.to_string()))
            .reduce(|acc, row| format!("{}{}", acc, row))
            .unwrap();
        format!("{}\n{}", headers, data)
    }
}
