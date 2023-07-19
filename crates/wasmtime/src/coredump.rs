use std::fmt;

use crate::{store::StoreOpaque, Global, Instance, Memory, Module, WasmBacktrace};

/// Representation of a core dump of a WebAssembly module
///
/// This structure is attached to the [`anyhow::Error`] returned from many
/// Wasmtime functions that execute WebAssembly such as [`Instance::new`] or
/// [`Func::call`]. This can be acquired with the [`anyhow::Error::downcast`]
/// family of methods to programmatically inspect the coredump. Otherwise since
/// it's part of the error returned this will get printed along with the rest of
/// the error when the error is logged.
///
/// TODO: Notably absent from this structure at the moment are any locals or
/// operand values for the stack frames. More work is needed to be able to
/// recover those when a trap occurs, at which point they will be added here.
///
/// Capturing of wasm coredumps can be configured through the
/// [`Config::coredump_on_trap`](crate::Config::coredump_on_trap) method.
///
/// For more information about errors in wasmtime see the documentation of the
/// [`Trap`] type.
///
/// [`Func::call`]: crate::Func::call
/// [`Instance::new`]: crate::Instance::new
pub struct WasmCoreDump {
    name: String,
    modules: Vec<Module>,
    instances: Vec<Instance>,
    memories: Vec<Memory>,
    globals: Vec<Global>,
    backtrace: WasmBacktrace,
}

impl WasmCoreDump {
    pub(crate) fn new(store: &StoreOpaque, backtrace: WasmBacktrace) -> WasmCoreDump {
        let modules: Vec<_> = store.modules().all_modules().cloned().collect();
        let instances: Vec<Instance> = store.all_instances().collect();
        let memories: Vec<Memory> = store.all_memories().collect();
        let globals: Vec<Global> = store.all_globals().collect();
        WasmCoreDump {
            name: String::from("store_name"),
            modules,
            instances,
            memories,
            globals,
            backtrace,
        }
    }
}

impl fmt::Display for WasmCoreDump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "wasm coredump generated while executing {}:", self.name)?;
        writeln!(f, "modules:")?;
        for module in self.modules.iter() {
            writeln!(f, "  {}", module.name().unwrap_or_default())?;
        }

        writeln!(f, "instances:")?;
        for instance in self.instances.iter() {
            writeln!(f, "  {:?}", instance)?;
        }

        writeln!(f, "memories:")?;
        for memory in self.memories.iter() {
            writeln!(f, "  {:?}", memory)?;
        }

        writeln!(f, "globals:")?;
        for global in self.globals.iter() {
            writeln!(f, "  {:?}", global)?;
        }

        writeln!(f, "backtrace:")?;
        write!(f, "{}", self.backtrace)?;

        Ok(())
    }
}
