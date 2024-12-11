use anyhow::{Result, anyhow};
use std::{cell::RefCell, iter::Scan, rc::Rc};

pub struct ErrorTracker {
    inner: Rc<RefCell<Result<()>>>,
}

impl ErrorTracker {
    pub fn new() -> Self {
        ErrorTracker {
            inner: Rc::new(RefCell::new(Ok(()))),
        }
    }

    pub fn rc_clone(&self) -> Self {
        ErrorTracker {
            inner: Rc::clone(&self.inner),
        }
    }

    pub fn log_error(&mut self, message: &'static str) {
        let mut err_ref = self.inner.borrow_mut();
        if err_ref.is_ok() {
            *err_ref = Err(anyhow!(message));
        }
    }

    pub fn check_status(self) -> Result<()> {
        Rc::into_inner(self.inner).unwrap().into_inner()
    }
}

impl Default for ErrorTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::type_complexity)]
pub trait ScanErrors: Iterator {
    fn scan_errors<T, E>(
        self,
        error_ref: &ErrorTracker,
        message: &'static str,
    ) -> Scan<Self, ErrorTracker, impl FnMut(&mut ErrorTracker, Result<T, E>) -> Option<T>>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
    {
        self.into_iter()
            .scan(error_ref.rc_clone(), move |err, item| match item {
                Ok(value) => Some(value),
                Err(_) => {
                    err.log_error(message);
                    None
                }
            })
    }
}

impl<I, T, E> ScanErrors for I where I: Iterator<Item = Result<T, E>> {}

pub trait Unless: Sized {
    fn unless(self, error: ErrorTracker) -> Result<Self> {
        error.check_status()?;
        Ok(self)
    }
}
impl<T: Sized> Unless for T {}
