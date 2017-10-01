mod req;
mod resp;
mod url;
mod method;
mod header;
mod version;
mod status;

pub use http::head::status::*;
pub use http::head::version::*;
pub use http::head::method::*;
pub use http::head::header::*;
pub use http::head::url::*;


pub enum Head {
    Req(),
    Resp(),
}