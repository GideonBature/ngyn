pub mod body;
pub mod context;
pub mod transformer;
pub mod uri;

use http_body_util::Full;
pub use hyper::body::Bytes;
pub use body::{ParseBody, ParseBytes};
pub use transformer::{Dto, Param, Query, Transducer, Transformer};
pub use uri::ToParts;

pub type NgynRequest = hyper::Request<hyper::body::Incoming>;
pub type NgynResponse = hyper::Response<Full<Bytes>>;
