use super::{simulate, JW_EMERGENCES};
use crate::{DataPoint, DataPointFrame, ProbDist};
use std::fmt::{Display, Formatter};

/// A [DataPoint] tagged for information about the simulation parameters that generated it
pub struct TaggedDataPoint<const NUM_GEN: usize> {
    data: DataPoint<NUM_GEN>,
    pop_0: u32,
    prob_capture: f64,
    mating_delay: f64,
    dd_span: u32,
}

impl<const NUM_GEN: usize> TaggedDataPoint<NUM_GEN> {
    pub fn new(
        data: DataPoint<NUM_GEN>,
        pop_0: u32,
        prob_capture: f64,
        mating_delay: f64,
        dd_span: u32,
    ) -> Self {
        Self {
            data,
            pop_0,
            prob_capture,
            mating_delay,
            dd_span,
        }
    }

    /// Creates a [TaggedDataPointFrame] from a [DataPointFrame]
    pub fn new_vec(
        data: DataPointFrame<NUM_GEN>,
        pop_0: u32,
        prob_capture: f64,
        mating_delay: f64,
        dd_span: u32,
    ) -> TaggedDataPointFrame<NUM_GEN> {
        TaggedDataPointFrame(
            data.0
                .into_iter()
                .map(|datapoint| Self {
                    data: datapoint,
                    pop_0,
                    prob_capture,
                    mating_delay,
                    dd_span,
                })
                .collect(),
        )
    }

    pub fn csv_headers(&self) -> String {
        format!(
            "{},pop_0,prob_capture,mating_delay,dd_span",
            self.data.csv_headers()
        )
    }
}

impl<const NUM_GEN: usize> Display for TaggedDataPoint<NUM_GEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.data.to_string(),
            self.pop_0,
            self.prob_capture,
            self.mating_delay,
            self.dd_span,
        )
    }
}

/// Newtype for a Vec of TaggedDataPoints
pub struct TaggedDataPointFrame<const NUM_GEN: usize>(pub Vec<TaggedDataPoint<NUM_GEN>>);

impl<const NUM_GEN: usize> TaggedDataPointFrame<NUM_GEN> {
    pub fn to_csv_string(&self) -> String {
        let headers = self
            .0
            .get(0)
            .expect("Couldn't get first TaggedDataPoint")
            .csv_headers();
        let data = self
            .0
            .iter()
            .enumerate()
            .map(|(idx, x)| format!("{}{}\n", idx % x.dd_span as usize, x.to_string()))
            .collect::<Vec<String>>()
            .join("");
        format!("{}\n{}", headers, data)
    }
}

impl<const NUM_GEN: usize> IntoIterator for TaggedDataPointFrame<NUM_GEN> {
    type Item = TaggedDataPoint<NUM_GEN>;
    type IntoIter = std::vec::IntoIter<TaggedDataPoint<NUM_GEN>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone)]
pub enum MultiParam {
    Range(std::ops::RangeInclusive<f64>, usize),
    Constant(f64),
}

impl std::iter::IntoIterator for MultiParam {
    type Item = f64;
    type IntoIter = std::iter::Successors<f64, impl FnMut(&f64) -> Option<f64>>;
    // This is to allow the creation of a range of floating point numbers
    fn into_iter(self) -> Self::IntoIter {
        let (start, end, step) = match self {
            MultiParam::Range(range, step) => (*range.start(), *range.end(), step as f64),
            MultiParam::Constant(val) => (val, val, 1.0),
        };
        std::iter::successors(Some(start), move |prev| {
            let next = prev + ((end - start) / step);
            (next < end).then_some(next)
        })
    }
}

/// Generates a 3 dimensional matrix of simulation results that vary across three degrees of freedom.
pub fn multisim<const NUM_GEN: usize>(
    pop_0: MultiParam,
    prob_detection: MultiParam,
    deg_day_range: std::ops::RangeInclusive<u32>,
    emergences: [ProbDist; NUM_GEN],
    mating_delay: MultiParam,
    egg_multiplier: impl Fn(f64) -> f64,
) -> TaggedDataPointFrame<NUM_GEN> {
    let multiframe: Vec<Vec<Vec<TaggedDataPointFrame<NUM_GEN>>>> = pop_0
        .clone()
        .into_iter()
        .map(|pop| {
            // println!("Starting outer iteration");
            prob_detection
                .clone()
                .into_iter()
                .map(|detection| {
                    // println!("starting inner iteration");
                    mating_delay
                        .clone()
                        .into_iter()
                        .map(|delay| {
                            // println!("Generating set with parameters pop: {pop}, detection: {detection}, and delay: {delay}");
                            TaggedDataPoint::new_vec(
                                simulate(
                                    pop as u32,
                                    detection,
                                    deg_day_range.clone(),
                                    emergences.clone(),
                                    delay,
                                    egg_multiplier(delay),
                                ),
                                pop as u32,
                                detection,
                                delay,
                                deg_day_range.end() - deg_day_range.start() + 1,
                            )
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let flattened_multiframe = multiframe
        .into_iter()
        .flatten()
        .flatten()
        .flatten()
        .collect();

    TaggedDataPointFrame(flattened_multiframe)
}
