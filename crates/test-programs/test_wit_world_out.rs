use preview2::bindings::cli_base::environment as __with_name0;
use preview2::bindings::io::streams as __with_name1;
use preview2::bindings::filesystem::filesystem as __with_name2;
use preview2::bindings::cli_base::preopens as __with_name3;
use preview2::bindings::cli_base::exit as __with_name4;
pub type OutputStream = __with_name1::OutputStream;
const _: () = {
  assert!(4 == <OutputStream as wasmtime::component::ComponentType>::SIZE32);
  assert!(4 == <OutputStream as wasmtime::component::ComponentType>::ALIGN32);
};
pub type DescriptorStat = __with_name2::DescriptorStat;
const _: () = {
  assert!(88 == <DescriptorStat as wasmtime::component::ComponentType>::SIZE32);
  assert!(8 == <DescriptorStat as wasmtime::component::ComponentType>::ALIGN32);
};
pub struct TestReactor {
  add_strings: wasmtime::component::Func,
  get_strings: wasmtime::component::Func,
  pass_an_imported_record: wasmtime::component::Func,
  write_strings_to: wasmtime::component::Func,
}
const _: () = {
  use wasmtime::component::__internal::anyhow;
  impl TestReactor {
    
    pub fn add_to_linker<T, U>(
    linker: &mut wasmtime::component::Linker<T>,
    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> wasmtime::Result<()>
    where U: wasi::clocks::wall_clock::Host + wasi::poll::poll::Host + Send, T: Send,
    {
      wasi::clocks::wall_clock::add_to_linker(linker, get)?;
      wasi::poll::poll::add_to_linker(linker, get)?;
      Ok(())
    }
    
    /// Instantiates the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    pub async fn instantiate_async<T :Send>(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    component: &wasmtime::component::Component,
    linker: &wasmtime::component::Linker<T>,
    ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
      let instance = linker.instantiate_async(&mut store, component).await?;
      Ok((Self::new(store, &instance)?, instance))
    }
    
    /// Instantiates a pre-instantiated module using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    pub async fn instantiate_pre<T :Send>(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    instance_pre: &wasmtime::component::InstancePre<T>,
    ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
      let instance = instance_pre.instantiate_async(&mut store).await?;
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
      
      let add_strings = *__exports.typed_func::<(&[&str], ), (u32, )>("add-strings")?.func();
      let get_strings = *__exports.typed_func::<(), (Vec<String>, )>("get-strings")?.func();
      let pass_an_imported_record = *__exports.typed_func::<(DescriptorStat, ), (String, )>("pass-an-imported-record")?.func();
      let write_strings_to = *__exports.typed_func::<(OutputStream, ), (Result<(),()>, )>("write-strings-to")?.func();
      Ok(TestReactor {
        add_strings,
        get_strings,
        pass_an_imported_record,
        write_strings_to,
      })
    }
    pub async fn call_add_strings<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: &[&str],) -> wasmtime::Result<u32> where <S as wasmtime::AsContext>::Data: Send {
      let callee = unsafe {
        wasmtime::component::TypedFunc::<(&[&str], ), (u32, )>::new_unchecked(self.add_strings)
      };
      let (ret0,) = callee.call_async(store.as_context_mut(), (arg0, )).await?;
      callee.post_return_async(store.as_context_mut()).await?;
      Ok(ret0)
    }
    pub async fn call_get_strings<S: wasmtime::AsContextMut>(&self, mut store: S, ) -> wasmtime::Result<Vec<String>> where <S as wasmtime::AsContext>::Data: Send {
      let callee = unsafe {
        wasmtime::component::TypedFunc::<(), (Vec<String>, )>::new_unchecked(self.get_strings)
      };
      let (ret0,) = callee.call_async(store.as_context_mut(), ()).await?;
      callee.post_return_async(store.as_context_mut()).await?;
      Ok(ret0)
    }
    pub async fn call_write_strings_to<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: OutputStream,) -> wasmtime::Result<Result<(),()>> where <S as wasmtime::AsContext>::Data: Send {
      let callee = unsafe {
        wasmtime::component::TypedFunc::<(OutputStream, ), (Result<(),()>, )>::new_unchecked(self.write_strings_to)
      };
      let (ret0,) = callee.call_async(store.as_context_mut(), (arg0, )).await?;
      callee.post_return_async(store.as_context_mut()).await?;
      Ok(ret0)
    }
    pub async fn call_pass_an_imported_record<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: DescriptorStat,) -> wasmtime::Result<String> where <S as wasmtime::AsContext>::Data: Send {
      let callee = unsafe {
        wasmtime::component::TypedFunc::<(DescriptorStat, ), (String, )>::new_unchecked(self.pass_an_imported_record)
      };
      let (ret0,) = callee.call_async(store.as_context_mut(), (arg0, )).await?;
      callee.post_return_async(store.as_context_mut()).await?;
      Ok(ret0)
    }
  }
};
pub mod wasi {
  pub mod clocks {
    
    #[allow(clippy::all)]
    pub mod wall_clock {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      /// A time and date in seconds plus nanoseconds.
      #[derive(wasmtime::component::ComponentType)]
      #[derive(wasmtime::component::Lift)]
      #[derive(wasmtime::component::Lower)]
      #[component(record)]
      #[derive(Copy, Clone)]
      pub struct Datetime {
        #[component(name = "seconds")]
        pub seconds: u64,
        #[component(name = "nanoseconds")]
        pub nanoseconds: u32,
      }
      impl core::fmt::Debug for Datetime {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("Datetime").field("seconds", &self.seconds).field("nanoseconds", &self.nanoseconds).finish()
        }
      }
      const _: () = {
        assert!(16 == <Datetime as wasmtime::component::ComponentType>::SIZE32);
        assert!(8 == <Datetime as wasmtime::component::ComponentType>::ALIGN32);
      };
      #[wasmtime::component::__internal::async_trait]
      pub trait Host {
        /// Read the current value of the clock.
        /// 
        /// This clock is not monotonic, therefore calling this function repeatedly
        /// will not necessarily produce a sequence of non-decreasing values.
        /// 
        /// The returned timestamps represent the number of seconds since
        /// 1970-01-01T00:00:00Z, also known as [POSIX's Seconds Since the Epoch],
        /// also known as [Unix Time].
        /// 
        /// The nanoseconds field of the output is always less than 1000000000.
        /// 
        /// [POSIX's Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16
        /// [Unix Time]: https://en.wikipedia.org/wiki/Unix_time
        async fn now(&mut self, ) -> wasmtime::Result<Datetime>;
        /// Query the resolution of the clock.
        /// 
        /// The nanoseconds field of the output is always less than 1000000000.
        async fn resolution(&mut self, ) -> wasmtime::Result<Datetime>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where T: Send, U: Host + Send,
      {
        
        let mut inst = linker.instance("wasi:clocks/wall-clock")?;
        inst.func_wrap_async("now", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | Box::new(async move { 
          let host = get(caller.data_mut());
          let r = host.now().await;
          Ok((r?,))
        }))?;
        inst.func_wrap_async("resolution", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | Box::new(async move { 
          let host = get(caller.data_mut());
          let r = host.resolution().await;
          Ok((r?,))
        }))?;
        Ok(())
      }
      
    }
    
  }
  pub mod poll {
    
    #[allow(clippy::all)]
    pub mod poll {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      /// A "pollable" handle.
      /// 
      /// This is conceptually represents a `stream<_, _>`, or in other words,
      /// a stream that one can wait on, repeatedly, but which does not itself
      /// produce any data. It's temporary scaffolding until component-model's
      /// async features are ready.
      /// 
      /// And at present, it is a `u32` instead of being an actual handle, until
      /// the wit-bindgen implementation of handles and resources is ready.
      /// 
      /// `pollable` lifetimes are not automatically managed. Users must ensure
      /// that they do not outlive the resource they reference.
      /// 
      /// This [represents a resource](https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources).
      pub type Pollable = u32;
      const _: () = {
        assert!(4 == <Pollable as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Pollable as wasmtime::component::ComponentType>::ALIGN32);
      };
      #[wasmtime::component::__internal::async_trait]
      pub trait Host {
        /// Dispose of the specified `pollable`, after which it may no longer
        /// be used.
        async fn drop_pollable(&mut self, this: Pollable,) -> wasmtime::Result<()>;
        /// Poll for completion on a set of pollables.
        /// 
        /// The "oneoff" in the name refers to the fact that this function must do a
        /// linear scan through the entire list of subscriptions, which may be
        /// inefficient if the number is large and the same subscriptions are used
        /// many times. In the future, this is expected to be obsoleted by the
        /// component model async proposal, which will include a scalable waiting
        /// facility.
        /// 
        /// The result list<bool> is the same length as the argument
        /// list<pollable>, and indicates the readiness of each corresponding
        /// element in that / list, with true indicating ready.
        async fn poll_oneoff(&mut self, in_: Vec<Pollable>,) -> wasmtime::Result<Vec<bool>>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where T: Send, U: Host + Send,
      {
        
        let mut inst = linker.instance("wasi:poll/poll")?;
        inst.func_wrap_async("drop-pollable", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Pollable, ) | Box::new(async move { 
          let host = get(caller.data_mut());
          let r = host.drop_pollable(arg0,).await;
          r
        }))?;
        inst.func_wrap_async("poll-oneoff", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Vec<Pollable>, ) | Box::new(async move { 
          let host = get(caller.data_mut());
          let r = host.poll_oneoff(arg0,).await;
          Ok((r?,))
        }))?;
        Ok(())
      }
      
    }
    
  }
}
