pub mod server;
pub mod response;
pub mod process;

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
