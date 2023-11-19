// For completeness, wrappers around all of tracing's public logging and span macros are provided,
// even if they are not used at the present time.
#![allow(unused_macros)]

macro_rules! debug {
    ($($arg:tt)+) => {
        {
            tracing::debug!($($arg)+);
        }
    }
}

macro_rules! debug_span {
    ($($arg:tt)*) => {
        {
            {
                let _span = tracing::debug_span!($($arg)+);
                _span.entered()
            }
        }
    }
}

macro_rules! error {
    ($($arg:tt)*) => {
        {
            tracing::error!($($arg)+);
        }
    }
}

macro_rules! error_span {
    ($($arg:tt)*) => {
        {
            {
                let _span = tracing::error_span!($($arg)+);
                _span.entered()
            }
        }
    }
}

macro_rules! info {
    ($($arg:tt)*) => {
        {
            tracing::info!($($arg)+);
        }
    }
}

macro_rules! info_span {
    ($($arg:tt)*) => {
        {
            {
                let _span = tracing::info_span!($($arg)+);
                _span.entered()
            }
        }
    }
}

macro_rules! trace {
    ($($arg:tt)*) => {
        {
            tracing::trace!($($arg)+);
        }
    }
}

macro_rules! trace_span {
    ($($arg:tt)*) => {
        {
            {
                let _span = tracing::trace_span!($($arg)+);
                _span.entered()
            }
        }
    }
}

macro_rules! span {
    ($($arg:tt)*) => {
        {
            {
                let _span = tracing::span!($($arg)+);
                _span.entered()
            }
        }
    }
}

macro_rules! warn {
    ($($arg:tt)*) => {
        {
            tracing::warn!($($arg)+);
        }
    }
}

macro_rules! warn_span {
    ($($arg:tt)*) => {
        {
            {
                let _span = tracing::warn_span!($($arg)+);
                _span.entered()
            }
        }
    }
}
