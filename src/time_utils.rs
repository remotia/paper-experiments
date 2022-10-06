#[macro_export]
macro_rules! time_start {
    ($id: expr) => {
        TimestampAdder::new(&format!("{}_start", $id))
    };
}

#[macro_export]
macro_rules! time_diff {
    ($id: expr) => {
        TimestampDiffCalculator::new(&format!("{}_start", $id), &format!("{}_time", $id))
    };
}
