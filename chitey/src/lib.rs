macro_rules! codegen_reexport {
    ($name:ident) => {
        #[cfg(feature = "macros")]
        pub use chitey_codegen::$name;
    };
}

// codegen_reexport!(main);
// codegen_reexport!(test);
codegen_reexport!(route);
codegen_reexport!(routes);
// codegen_reexport!(head);
codegen_reexport!(get);
codegen_reexport!(post);
// codegen_reexport!(patch);
// codegen_reexport!(put);
// codegen_reexport!(delete);
// codegen_reexport!(trace);
// codegen_reexport!(connect);
// codegen_reexport!(options);

pub use chitey_server::guard::Guard;
pub use chitey_server::resource::Resource;
pub use chitey_server::resource::Responder;
pub use chitey_server::web_server::Request;
pub use chitey_server::server::util::TlsCertsKey;
pub use chitey_server::web_server::Certs;
pub use chitey_server::web_server::HttpServiceFactory;
pub use chitey_server::web_server::WebServer;
pub use async_trait::async_trait;
// pub use chitey_server::tuple::Path;
// pub use chitey_server::tuple::Tuple;
pub use chitey_server::web_server::ChiteyError;
pub use urlpattern::UrlPatternMatchInput;
pub use tokio;
pub use tokio::main;
pub use chitey_server::process::kill_server;
pub use http;
pub use hyper;
pub use hyper::Body;
pub use bytes;
pub use bytes::Bytes;
