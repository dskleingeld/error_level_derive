use error_level::ErrorLevel;
use error_level_derive::ErrorLevel;
use log::Level;

#[test]
fn it_works() {

    #[allow(dead_code)]
    #[derive(ErrorLevel)]
    pub enum OuterError {
        #[level(Info)]
        Error0,
        Error1,
    }

    #[derive(ErrorLevel)]
    pub enum CustomError {
        #[level(Warn)]
        ErrorA,
        #[level(Info)]
        ErrorB,
        #[level(No)]
        ErrorC,
        ErrorD(OuterError),
    };

    let a = CustomError::ErrorA;
    let b = CustomError::ErrorB;
    let c = CustomError::ErrorC;
    let d = CustomError::ErrorD(OuterError::Error0);

    //TODO move to directly reporting instead of returning a level
    //can implement both as separate Traits, one returning Option<Level>
    //and one using that to actually report
    assert_eq!(a.error_level(), Level::Warn);
    assert_eq!(b.error_level(), Level::Warn);
}
