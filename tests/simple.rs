use error_level::ErrorLevel;

use error_level_derive::ErrorLevel;
use log::Level;
use simplelog::{LevelFilter, Config, SimpleLogger};

#[allow(dead_code)]
#[derive(Debug, ErrorLevel)]
pub enum OuterError {
    #[level(Info)]
    Error0,
    // Error1,
}

#[derive(Debug)]
enum ErrorWithoutImpl {
    Error0,
    Error1,
}

#[test]
fn it_works() {
    #[derive(Debug, ErrorLevel)]
    pub enum CustomError {
        #[level(Warn)]
        ErrorA,
        #[level(Info)]
        ErrorB,
        #[level(No)]
        ErrorC,
        ErrorD(OuterError),
    }

    SimpleLogger::init(LevelFilter::Trace, Config::default()).unwrap();

    let a = CustomError::ErrorA;
    let b = CustomError::ErrorB;
    let c = CustomError::ErrorC;
    let d = CustomError::ErrorD(OuterError::Error0);

    assert_eq!(a.error_level(), Some(Level::Warn));
    assert_eq!(b.error_level(), Some(Level::Warn));
    b.log_error();
}

#[test]
fn does_not_implement_ErrorLevel() {
    

    #[derive(Debug, ErrorLevel)]
    pub enum CustomError {
        #[level(Warn)]
        ErrorA,
        #[level(Info)]
        ErrorB,
        #[level(No)]
        ErrorC,
        ErrorD(ErrorWithoutImpl),
    }

    let a = CustomError::ErrorA;
    let d = CustomError::ErrorD(ErrorWithoutImpl::Error1);
}

#[test]
fn should_not_compile() {
    #[derive(Debug, ErrorLevel)]
    pub enum CustomError {
        #[level(Warn)]
        ErrorA,
        #[level(Info)]
        ErrorB,
        ErrorC,
        ErrorD((String, String)),
    }
}
