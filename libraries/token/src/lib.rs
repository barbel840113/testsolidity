pub use fadroma;

pub use token_pair::*;
pub use token_type::*;
pub use token_pair_amount::*;
pub use token_type_amount::*;

#[cfg(not(target_arch = "wasm32"))]
// This is instead of declaring it as a testing package due to limit of re-exporting testing packages

mod token_pair;
mod token_type;
mod token_pair_amount;
mod token_type_amount;