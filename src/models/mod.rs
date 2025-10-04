pub mod sighting;
pub mod taxon;
pub mod trip;

// optional re-exports so you can just `use crate::models::Sighting;`
pub use sighting::Sighting;
pub use taxon::Taxon;
pub use trip::Trip;
