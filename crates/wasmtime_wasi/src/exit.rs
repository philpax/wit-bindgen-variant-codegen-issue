use super::{wasi_exit, WasiCtx};

impl wasi_exit::Host for WasiCtx {
    fn exit(&mut self, status: Result<(), ()>) -> anyhow::Result<()> {
        let status = match status {
            Ok(()) => 0,
            Err(()) => 1,
        };
        Err(anyhow::anyhow!(wasi_common::I32Exit(status)))
    }
}
