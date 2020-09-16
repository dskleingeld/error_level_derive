use error_level::ErrorLevel;

use error_level_derive::ErrorLevel;
use log::Level;
use simplelog::{LevelFilter, Config, SimpleLogger};

#[allow(dead_code)]
// #[derive(Debug, ErrorLevel)]
#[derive(Debug)]
pub enum OuterError {
    // #[level(Info)]
    Error0,
    // Error1,
}

#[derive(Debug, ErrorLevel)]
pub enum CustomError {
    // #[level(Warn)]
    // ErrorA,
    // #[level(Info)]
    // ErrorB,
    // #[level(No)]
    // ErrorC,
    ErrorD(OuterError),
    ErrorE((String, String)),
    ErrorF(String),
}

#[test]
fn it_works() {
    SimpleLogger::init(LevelFilter::Trace, Config::default()).unwrap();

    // let a = CustomError::ErrorA;
    // let b = CustomError::ErrorB;
    // let c = CustomError::ErrorC;
    let d = CustomError::ErrorD(OuterError::Error0);

    //TODO move to directly reporting instead of returning a level
    //can implement both as separate Traits, one returning Option<Level>
    //and one using that to actually report
    // assert_eq!(a.error_level(), Some(Level::Warn));
    // assert_eq!(b.error_level(), Some(Level::Warn));
    // b.log_error();
}
