use snafu::Location;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("No calculation formula for wcu set!"))]
    WcuCalcNotSet { location: Location },

    #[snafu(display("No calculation formula for rcu set!"))]
    RcuCalcNotSet { location: Location },

    #[snafu(whatever, display("{message}"))]
    Generic {
        message: String,

        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error1 {
    #[snafu(display("No calculation formula for wcu set!"))]
    WcuCalc { location: Location },
}

pub type Result<T> = std::result::Result<T, Error>;
