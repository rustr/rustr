use rdll::*;
use traits::*;

use chrono::*;
use error::*;
use rtype::*;
use rcast::*;
pub struct RDate {
    m_d: f64,
    m_t: Date<FixedOffset>,
}

impl RDate {
    pub fn new(x: SEXP) -> RResult<RDate> {
        let double: f64 = try!(RNew::rnew(try!(r_cast(x, REALSXP))));
        let time = try!(DateTime::parse_from_str(format!("{}", double).as_ref(), "%s")).date();
        Ok(RDate {
            m_d: double,
            m_t: time,
        })
    }

    pub fn update_t(&mut self) -> RResult<()> {
        self.m_t = try!(DateTime::parse_from_str(format!("{}", self.m_d).as_ref(), "%s")).date();
        Ok(())
    }
}


impl From<::chrono::format::ParseError> for RError {
    fn from(x: ::chrono::format::ParseError) -> RError {
        RError::new(REKind::Other(x.into()))
    }
}
