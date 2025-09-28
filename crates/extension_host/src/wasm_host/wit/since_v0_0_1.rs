use semantic_version::SemanticVersion;
use wasmtime::component::{Component, Linker, bindgen};
use wasmtime::{Config, Engine, Store};

pub const MIN_VERSION: SemanticVersion = SemanticVersion::new(0, 0, 1);

bindgen!({
    path: "../extension_api/wit/since_v0.0.1",
    world: "extension",
});
