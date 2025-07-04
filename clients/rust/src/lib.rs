mod generated;

use generated::*;

pub mod accounts {
    pub use super::generated::accounts::*;
}

pub mod instructions {
    pub use super::generated::instructions::*;
}

pub mod errors {
    #[allow(unused_imports)]
    pub use super::generated::errors::*;
}

pub mod shared {
    #[allow(unused_imports)]
    pub use super::generated::shared::*;
}

pub mod programs {
    pub use super::generated::programs::*;
}
