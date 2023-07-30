pub struct TheWorld {
  interface0: exports::foo::foo::resources::Resources,
}
const _: () = {
  use wasmtime::component::__internal::anyhow;
  impl TheWorld {
    
    pub fn add_to_linker<T, U>(
    linker: &mut wasmtime::component::Linker<T>,
    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> wasmtime::Result<()>
    where U: foo::foo::resources::Host,
    {
      foo::foo::resources::add_to_linker(linker, get)?;
      Ok(())
    }
    
    /// Instantiates the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    pub  fn instantiate<T >(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    component: &wasmtime::component::Component,
    linker: &wasmtime::component::Linker<T>,
    ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
      let instance = linker.instantiate(&mut store, component)?;
      Ok((Self::new(store, &instance)?, instance))
    }
    
    /// Instantiates a pre-instantiated module using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    pub  fn instantiate_pre<T >(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    instance_pre: &wasmtime::component::InstancePre<T>,
    ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
      let instance = instance_pre.instantiate(&mut store)?;
      Ok((Self::new(store, &instance)?, instance))
    }
    
    /// Low-level creation wrapper for wrapping up the exports
    /// of the `instance` provided in this structure of wasm
    /// exports.
    ///
    /// This function will extract exports from the `instance`
    /// defined within `store` and wrap them all up in the
    /// returned structure which can be used to interact with
    /// the wasm module.
    pub fn new(
    mut store: impl wasmtime::AsContextMut,
    instance: &wasmtime::component::Instance,
    ) -> wasmtime::Result<Self> {
      let mut store = store.as_context_mut();
      let mut exports = instance.exports(&mut store);
      let mut __exports = exports.root();
      
      let interface0 = exports::foo::foo::resources::Resources::new(
      &mut __exports.instance("foo:foo/resources")
      .ok_or_else(|| anyhow::anyhow!("exported instance `foo:foo/resources` not present"))?
      )?;
      Ok(TheWorld {
        interface0,
      })
    }
    
    pub fn foo_foo_resources(&self) -> &exports::foo::foo::resources::Resources {
      &self.interface0
    }
  }
};
pub mod foo {
  pub mod foo {
    
    #[allow(clippy::all)]
    pub mod resources {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      /// A resource containing two scalar fields
      /// that both have the same type
      pub struct ScalarsImpl;
      pub trait Scalars {
        /// constructor
        fn constructor_scalars(&mut self, init: Vec<u8>,) -> wasmtime::Result<wasmtime::component::Resource<ScalarsImpl>>;
        /// The first field, named a
        fn static_scalars_get_a(&mut self, ) -> wasmtime::Result<u32>;
        /// The second field, named b
        fn method_scalars_get_b(&mut self, self_: wasmtime::component::Resource<ScalarsImpl>,) -> wasmtime::Result<u32>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Scalars,
      {
        
        let mut inst = linker.instance("scalars")?;
        inst.func_wrap("[constructor]scalars", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Vec<u8>, ) | { 
          
          let span = tracing::span!(
          tracing::Level::TRACE,
          "wit-bindgen import",
          module = "resources",
          function = "[constructor]scalars",
          );
          let _enter = span.enter();
          tracing::event!(tracing::Level::TRACE, init = tracing::field::debug(&arg0), "call");
          let host = get(caller.data_mut());
          let r = host.constructor_scalars(arg0,);
          tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");Ok((r?,))
        })?;
        inst.func_wrap("[static]scalars.get-a", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
          
          let span = tracing::span!(
          tracing::Level::TRACE,
          "wit-bindgen import",
          module = "resources",
          function = "[static]scalars.get-a",
          );
          let _enter = span.enter();
          tracing::event!(tracing::Level::TRACE, "call");
          let host = get(caller.data_mut());
          let r = host.static_scalars_get_a();
          tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");Ok((r?,))
        })?;
        inst.func_wrap("[method]scalars.get-b", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<ScalarsImpl>, ) | { 
          
          let span = tracing::span!(
          tracing::Level::TRACE,
          "wit-bindgen import",
          module = "resources",
          function = "[method]scalars.get-b",
          );
          let _enter = span.enter();
          tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
          let host = get(caller.data_mut());
          let r = host.method_scalars_get_b(arg0,);
          tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");Ok((r?,))
        })?;
        Ok(())
      }
      pub trait Host {
        fn scalar_arg(&mut self, x: wasmtime::component::Resource<ScalarsImpl>,) -> wasmtime::Result<()>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("foo:foo/resources")?;
        inst.func_wrap("scalar-arg", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<ScalarsImpl>, ) | { 
          
          let span = tracing::span!(
          tracing::Level::TRACE,
          "wit-bindgen import",
          module = "resources",
          function = "scalar-arg",
          );
          let _enter = span.enter();
          tracing::event!(tracing::Level::TRACE, x = tracing::field::debug(&arg0), "call");
          let host = get(caller.data_mut());
          let r = host.scalar_arg(arg0,);
          tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");r
        })?;
        Ok(())
      }
      
    }
    
  }
}
pub mod exports {
  pub mod foo {
    pub mod foo {
      
      #[allow(clippy::all)]
      pub mod resources {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::anyhow;
        
        /// A resource containing two scalar fields
        /// that both have the same type
        pub struct ScalarsImpl;
        pub trait Scalars {
          /// constructor
          fn constructor_scalars(&mut self, init: Vec<u8>,) -> wasmtime::Result<wasmtime::component::Resource<ScalarsImpl>>;
          /// The first field, named a
          fn static_scalars_get_a(&mut self, ) -> wasmtime::Result<u32>;
          /// The second field, named b
          fn method_scalars_get_b(&mut self, self_: wasmtime::component::Resource<ScalarsImpl>,) -> wasmtime::Result<u32>;
        }
        
        pub fn add_to_linker<T, U>(
        linker: &mut wasmtime::component::Linker<T>,
        get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wasmtime::Result<()>
        where U: Scalars,
        {
          
          let mut inst = linker.instance("scalars")?;
          inst.func_wrap("[constructor]scalars", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Vec<u8>, ) | { 
            
            let span = tracing::span!(
            tracing::Level::TRACE,
            "wit-bindgen import",
            module = "resources",
            function = "[constructor]scalars",
            );
            let _enter = span.enter();
            tracing::event!(tracing::Level::TRACE, init = tracing::field::debug(&arg0), "call");
            let host = get(caller.data_mut());
            let r = host.constructor_scalars(arg0,);
            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");Ok((r?,))
          })?;
          inst.func_wrap("[static]scalars.get-a", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
            
            let span = tracing::span!(
            tracing::Level::TRACE,
            "wit-bindgen import",
            module = "resources",
            function = "[static]scalars.get-a",
            );
            let _enter = span.enter();
            tracing::event!(tracing::Level::TRACE, "call");
            let host = get(caller.data_mut());
            let r = host.static_scalars_get_a();
            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");Ok((r?,))
          })?;
          inst.func_wrap("[method]scalars.get-b", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<ScalarsImpl>, ) | { 
            
            let span = tracing::span!(
            tracing::Level::TRACE,
            "wit-bindgen import",
            module = "resources",
            function = "[method]scalars.get-b",
            );
            let _enter = span.enter();
            tracing::event!(tracing::Level::TRACE, self_ = tracing::field::debug(&arg0), "call");
            let host = get(caller.data_mut());
            let r = host.method_scalars_get_b(arg0,);
            tracing::event!(tracing::Level::TRACE, result = tracing::field::debug(&r), "return");Ok((r?,))
          })?;
          Ok(())
        }
        pub struct Resources {
          constructor_scalars: wasmtime::component::Func,
          static_scalars_get_a: wasmtime::component::Func,
          method_scalars_get_b: wasmtime::component::Func,
          scalar_arg: wasmtime::component::Func,
        }
        impl Resources {
          
          pub fn new(
          __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
          ) -> wasmtime::Result<Resources> {
            let constructor_scalars = *__exports.typed_func::<(&[u8], ), (wasmtime::component::Resource<ScalarsImpl>, )>("[constructor]scalars")?.func();
            let static_scalars_get_a = *__exports.typed_func::<(), (u32, )>("[static]scalars.get-a")?.func();
            let method_scalars_get_b = *__exports.typed_func::<(wasmtime::component::Resource<ScalarsImpl>, ), (u32, )>("[method]scalars.get-b")?.func();
            let scalar_arg = *__exports.typed_func::<(wasmtime::component::Resource<ScalarsImpl>, ), ()>("scalar-arg")?.func();
            Ok(Resources {
              constructor_scalars,
              static_scalars_get_a,
              method_scalars_get_b,
              scalar_arg,
            })
          }
          /// constructor
          pub  fn call_constructor_scalars<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: &[u8],) -> wasmtime::Result<wasmtime::component::Resource<ScalarsImpl>> {
            
            let span = tracing::span!(
            tracing::Level::TRACE,
            "wit-bindgen export",
            module = "foo:foo/resources",
            function = "[constructor]scalars",
            );
            let _enter = span.enter();
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(&[u8], ), (wasmtime::component::Resource<ScalarsImpl>, )>::new_unchecked(self.constructor_scalars)
            };
            let (ret0,) = callee.call(store.as_context_mut(), (arg0, ))?;
            callee.post_return(store.as_context_mut())?;
            Ok(ret0)
          }
          /// The first field, named a
          pub  fn call_static_scalars_get_a<S: wasmtime::AsContextMut>(&self, mut store: S, ) -> wasmtime::Result<u32> {
            
            let span = tracing::span!(
            tracing::Level::TRACE,
            "wit-bindgen export",
            module = "foo:foo/resources",
            function = "[static]scalars.get-a",
            );
            let _enter = span.enter();
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(), (u32, )>::new_unchecked(self.static_scalars_get_a)
            };
            let (ret0,) = callee.call(store.as_context_mut(), ())?;
            callee.post_return(store.as_context_mut())?;
            Ok(ret0)
          }
          /// The second field, named b
          pub  fn call_method_scalars_get_b<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: wasmtime::component::Resource<ScalarsImpl>,) -> wasmtime::Result<u32> {
            
            let span = tracing::span!(
            tracing::Level::TRACE,
            "wit-bindgen export",
            module = "foo:foo/resources",
            function = "[method]scalars.get-b",
            );
            let _enter = span.enter();
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(wasmtime::component::Resource<ScalarsImpl>, ), (u32, )>::new_unchecked(self.method_scalars_get_b)
            };
            let (ret0,) = callee.call(store.as_context_mut(), (arg0, ))?;
            callee.post_return(store.as_context_mut())?;
            Ok(ret0)
          }
          pub  fn call_scalar_arg<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: wasmtime::component::Resource<ScalarsImpl>,) -> wasmtime::Result<()> {
            
            let span = tracing::span!(
            tracing::Level::TRACE,
            "wit-bindgen export",
            module = "foo:foo/resources",
            function = "scalar-arg",
            );
            let _enter = span.enter();
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(wasmtime::component::Resource<ScalarsImpl>, ), ()>::new_unchecked(self.scalar_arg)
            };
            let () = callee.call(store.as_context_mut(), (arg0, ))?;
            callee.post_return(store.as_context_mut())?;
            Ok(())
          }
        }
        
      }
      
    }
  }
}
