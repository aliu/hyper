macro_rules! ready {
    ($e:expr) => {
        match $e {
            std::task::Poll::Ready(v) => v,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}

pub(crate) mod buf;
pub(crate) mod date;
pub(crate) mod io;
pub(crate) mod task;
pub(crate) mod time;
pub(crate) mod watch;
