use semantic_version::SemanticVersion;
use wasmtime::component::{Component, Linker, bindgen};
use wasmtime::{Config, Engine, Store};

pub const MIN_VERSION: SemanticVersion = SemanticVersion::new(0, 0, 1);

bindgen!({
    path: "../extension_api/wit/since_v0.0.1",
    world: "extension",
});


fn foo() {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, ());
}