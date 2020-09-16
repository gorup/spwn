#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! spwn {
        ($task:expr) => {
            $crate::spwn($task);
        };
    }

    #[macro_export]
    macro_rules! spwn_blk {
        ($task:expr) => {
            $crate::spwn_blk($task);
        };
    }
}
