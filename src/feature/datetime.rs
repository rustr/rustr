
use chrono::*;
use error::*;
use rtype::*;
use traits::*;
use rdll::*;
use r_cast::*;

pub struct RDateTime {
    m_d: f64,
    m_t: DateTime<FixedOffset>,
}

impl RDateTime {
    pub fn new(x: SEXP) -> RResult<RDateTime> {
        let double: f64 = try!(RNew::rnew(try!(r_cast(x, REALSXP))));
        let time = try!(DateTime::parse_from_str(format!("{}", double).as_ref(), "%s"));
        Ok(RDateTime {
            m_d: double,
            m_t: time,
        })
    }

    pub fn update_t(&mut self) -> RResult<()> {
        self.m_t = try!(DateTime::parse_from_str(format!("{}", self.m_d).as_ref(), "%s"));
        Ok(())
    }
}
