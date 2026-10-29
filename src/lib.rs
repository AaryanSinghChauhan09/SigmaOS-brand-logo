//! SigmaOS Brand Logo
//! Brand assets and logo management

pub mod branding;

pub use branding::{Branding, MAX_IDENTITY_LEN, MAX_LOGO_LINES};
pub type BrandManager = Branding;
