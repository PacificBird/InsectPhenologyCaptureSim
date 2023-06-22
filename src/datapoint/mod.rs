use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug)]
pub struct DataPoint<const NUM_GEN: usize> {
    pub pop_captured: f64,
    pub pop_active: [f64; NUM_GEN],
    pub eggs: [f64; NUM_GEN],
    pub eggs_total: [f64; NUM_GEN],
}
impl<const NUM_GEN: usize> Display for DataPoint<NUM_GEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let pop_active_string: String = self
            .pop_active
            .clone()
            .into_iter()
            .fold("".to_owned(), |acc, x| format!("{}{},", acc, x));

        let eggs_string: String = self
            .eggs
            .clone()
            .into_iter()
            .fold("".to_owned(), |acc, x| format!("{}{},", acc, x));

        let eggs_total_string: String = self
            .eggs_total
            .clone()
            .into_iter()
            .fold("".to_owned(), |acc, x| format!("{}{},", acc, x));

        write!(
            f,
            "{},{}{}{}",
            self.pop_captured, pop_active_string, eggs_string, eggs_total_string
        )
    }
}
impl<const NUM_GEN: usize> DataPoint<NUM_GEN> {
    pub fn csv_headers(&self) -> String {
        let pop_headers = self
            .pop_active
            .clone()
            .into_iter()
            .enumerate()
            .fold("".to_owned(), |acc, (i, _)| {
                format!("{acc}pop_active_{},", i)
            });
        let egg_headers = self
            .eggs
            .clone()
            .into_iter()
            .enumerate()
            .fold("".to_owned(), |acc, (i, _)| format!("{acc}eggs_{},", i));
        let egg_total_headers = self
            .eggs_total
            .clone()
            .into_iter()
            .enumerate()
            .fold("".to_owned(), |acc, (i, _)| {
                format!("{acc}eggs_total_{},", i)
            });

        format!("dd,captured,{pop_headers}{egg_headers}{egg_total_headers}")
    }
}

pub struct DataPointFrame<const NUM_GEN: usize>(pub Vec<DataPoint<NUM_GEN>>);

impl<const NUM_GEN: usize> DataPointFrame<NUM_GEN> {
    pub fn to_csv_string(&self) -> String {
        let headers = self.0.get(1).unwrap().csv_headers();
        let data = self
            .0
            .iter()
            .enumerate()
            .map(|(idx, x)| format!("{},{}\n", idx, x.to_string()))
            .reduce(|acc, row| format!("{}{}", acc, row))
            .unwrap();
        format!("{}\n{}", headers, data)
    }
}
