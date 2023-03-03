use std::{any::Any, path::PathBuf};

wasmtime::component::bindgen!("main.server");

struct ServerImpl;
impl types::Host for ServerImpl {}
impl component::Host for ServerImpl {
    fn get_component(
        &mut self,
        _: types::EntityId,
        _: u32,
    ) -> anyhow::Result<Option<component::ComponentTypeResult>> {
        todo!()
    }
    fn add_component(
        &mut self,
        _: types::EntityId,
        _: u32,
        _: component::ComponentTypeResult,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
impl entity::Host for ServerImpl {
    fn spawn(
        &mut self,
        args: Vec<(u32, component::ComponentTypeResult)>,
    ) -> anyhow::Result<types::EntityId> {
        println!("---");
        println!("Host: {args:?}");
        println!("---");
        anyhow::bail!("Unimplemented");
    }
}

struct State {
    wasi_ctx: ambient_wasmtime_wasi::WasiCtx,
    server_impl: ServerImpl,
}

struct WasiOutputStream;
#[async_trait::async_trait]
impl wasi_common::OutputStream for WasiOutputStream {
    fn as_any(&self) -> &dyn Any {
        self
    }
    async fn writable(&self) -> Result<(), wasi_common::Error> {
        Ok(())
    }

    async fn write(&mut self, buf: &[u8]) -> Result<u64, wasi_common::Error> {
        let msg = std::str::from_utf8(buf).unwrap();
        print!("{msg}");
        Ok(buf.len().try_into()?)
    }
}

fn main() -> anyhow::Result<()> {
    let input = PathBuf::from("guest/guest.wasm");

    let mut config = wasmtime::Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);

    let engine = wasmtime::Engine::new(&config)?;
    let component = wasmtime::component::Component::from_file(&engine, &input)?;
    let mut linker = wasmtime::component::Linker::new(&engine);
    ambient_wasmtime_wasi::add_to_linker(&mut linker, |x: &mut State| &mut x.wasi_ctx)?;
    Server::add_to_linker(&mut linker, |x: &mut State| &mut x.server_impl)?;

    let mut store = wasmtime::Store::new(
        &engine,
        State {
            wasi_ctx: wasi_cap_std_sync::WasiCtxBuilder::new()
                .stdout(Box::new(WasiOutputStream))
                .stderr(Box::new(WasiOutputStream))
                .build(),
            server_impl: ServerImpl,
        },
    );

    let (bindings, _) = Server::instantiate(&mut store, &component, &linker)?;
    bindings.guest.call_init(&mut store)?;

    Ok(())
}
