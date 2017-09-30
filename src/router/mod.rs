pub mod segment;
pub mod router;
pub mod priority_router;
pub mod route;

use context::Context;
pub use router::router::Router;

pub type Handler = Fn(&mut Context) + Send + Sync +'static;
