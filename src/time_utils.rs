#[macro_export]
macro_rules! time_start {
    ($id: expr) => {
        remotia::time::add::TimestampAdder::new(&format!("{}_start", $id))
    };
}

#[macro_export]
macro_rules! time_diff {
    ($id: expr) => {
        remotia::time::diff::TimestampDiffCalculator::new(&format!("{}_start", $id), &format!("{}_time", $id))
    };
}
