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

macro_rules! router_reexport {
    ($name:ident) => {
        pub use chitey_router::resource::$name;
    };
}
router_reexport!(Resource);
// router_reexport!(HttpServiceFactory);
// router_reexport!(HandleRegister);
router_reexport!(Responder);
pub use chitey_server::guard::Guard;
pub use chitey_server::server::util::TlsCertsKey;
pub use chitey_server::web_server::Certs;
pub use chitey_server::web_server::HttpServiceFactory;
pub use chitey_server::web_server::WebServer;
