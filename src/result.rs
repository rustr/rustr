//! Safe unwrap for R ffi
//!
use ::traits::*;

use std::result::Result::{self, Ok, Err};
use std::option::Option::{self, Some, None};


impl<T: Empty, E> Gettable for Result<T, E> {
    type Res=T;
    /// Safely unwraps a Result, yielding the content of an `Ok`.
    ///
    /// # Examples
    ///
    /// ```txt
    /// let x: Rresult<u32, &str> = Ok(2);
    /// assert_eq!(x.unwrap(), 2);
    /// ```
    ///
    /// ```txt
    /// let x: Rresult<u32, &str> = Err("emergency failure");
    /// x.get();
    /// ```
    #[inline]
    fn get(self) -> T {
        match self {
            Ok(t) => t,
            Err(_) => T::empty(),
            // panic!("called `Rresult::unwrap()` on an `Err` value: {:?}", e)
        }
    }
}

impl<T, E: Empty> ErrGettable for Result<T, E> {
    type Res=E;
    /// Safely unwraps a Result, yielding the content of an `Err`.
    ///
    /// # Examples
    ///
    /// ```txt
    /// let x: Rresult<u32, &str> = Ok(2);
    /// x.get_err(); 
    /// ```
    ///
    /// ```txt
    /// let x: Rresult<u32, &str> = Err("emergency failure");
    /// assert_eq!(x.get_err(), "emergency failure");
    /// ```
    #[inline]
    fn get_err(self) -> E {
        match self {
            Ok(_) => E::empty(),
            Err(e) => e,
        }
    }
}


impl<T: Empty> Gettable for Option<T> {
    type Res=T;
    /// Moves the value `v` out of the `Roption<T>` if it is `Some(v)`.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals `None`.
    ///
    /// # Safety note
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the `None`
    /// case explicitly.
    ///
    /// # Examples
    ///
    /// ```txt
    /// let x = Some("air");
    /// assert_eq!(x.get(), "air");
    /// ```
    ///
    /// ```txt
    /// let x: Roption<&str> = None;
    /// assert_eq!(x.get(), "air"); // fails
    /// ```
    #[inline]
    fn get(self) -> T {
        match self {
            Some(val) => val,
            None => T::empty(),
        }
    }
}
