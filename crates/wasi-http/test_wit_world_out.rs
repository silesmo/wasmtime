pub struct Proxy {
  interface0: exports::wasi::http::incoming_handler::IncomingHandler,
}
const _: () = {
  use wasmtime::component::__internal::anyhow;
  impl Proxy {
    
    pub fn add_to_linker<T, U>(
    linker: &mut wasmtime::component::Linker<T>,
    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> wasmtime::Result<()>
    where U: wasi::cli::stdout::Host + wasi::cli::stderr::Host + wasi::cli::stdin::Host + wasi::clocks::wall_clock::Host + wasi::clocks::monotonic_clock::Host + wasi::clocks::timezone::Host + wasi::http::types::Host + wasi::http::outgoing_handler::Host + wasi::io::streams::Host + wasi::poll::poll::Host + wasi::random::random::Host,
    {
      wasi::cli::stdout::add_to_linker(linker, get)?;
      wasi::cli::stderr::add_to_linker(linker, get)?;
      wasi::cli::stdin::add_to_linker(linker, get)?;
      wasi::clocks::wall_clock::add_to_linker(linker, get)?;
      wasi::clocks::monotonic_clock::add_to_linker(linker, get)?;
      wasi::clocks::timezone::add_to_linker(linker, get)?;
      wasi::http::types::add_to_linker(linker, get)?;
      wasi::http::outgoing_handler::add_to_linker(linker, get)?;
      wasi::io::streams::add_to_linker(linker, get)?;
      wasi::poll::poll::add_to_linker(linker, get)?;
      wasi::random::random::add_to_linker(linker, get)?;
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
      
      let interface0 = exports::wasi::http::incoming_handler::IncomingHandler::new(
      &mut __exports.instance("wasi:http/incoming-handler")
      .ok_or_else(|| anyhow::anyhow!("exported instance `wasi:http/incoming-handler` not present"))?
      )?;
      Ok(Proxy {
        interface0,
      })
    }
    
    pub fn wasi_http_incoming_handler(&self) -> &exports::wasi::http::incoming_handler::IncomingHandler {
      &self.interface0
    }
  }
};
pub mod wasi {
  pub mod cli {
    
    #[allow(clippy::all)]
    pub mod stdout {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
      const _: () = {
        assert!(4 == <OutputStream as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <OutputStream as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub trait Host {
        fn get_stdout(&mut self, ) -> wasmtime::Result<OutputStream>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:cli/stdout")?;
        inst.func_wrap("get-stdout", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
          let host = get(caller.data_mut());
          let r = host.get_stdout();
          Ok((r?,))
        })?;
        Ok(())
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod stderr {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
      const _: () = {
        assert!(4 == <OutputStream as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <OutputStream as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub trait Host {
        fn get_stderr(&mut self, ) -> wasmtime::Result<OutputStream>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:cli/stderr")?;
        inst.func_wrap("get-stderr", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
          let host = get(caller.data_mut());
          let r = host.get_stderr();
          Ok((r?,))
        })?;
        Ok(())
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod stdin {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      pub type InputStream = super::super::super::wasi::io::streams::InputStream;
      const _: () = {
        assert!(4 == <InputStream as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <InputStream as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub trait Host {
        fn get_stdin(&mut self, ) -> wasmtime::Result<InputStream>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:cli/stdin")?;
        inst.func_wrap("get-stdin", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
          let host = get(caller.data_mut());
          let r = host.get_stdin();
          Ok((r?,))
        })?;
        Ok(())
      }
      
    }
    
  }
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
        fn now(&mut self, ) -> wasmtime::Result<Datetime>;
        /// Query the resolution of the clock.
        /// 
        /// The nanoseconds field of the output is always less than 1000000000.
        fn resolution(&mut self, ) -> wasmtime::Result<Datetime>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:clocks/wall-clock")?;
        inst.func_wrap("now", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
          let host = get(caller.data_mut());
          let r = host.now();
          Ok((r?,))
        })?;
        inst.func_wrap("resolution", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
          let host = get(caller.data_mut());
          let r = host.resolution();
          Ok((r?,))
        })?;
        Ok(())
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod monotonic_clock {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      pub type Pollable = super::super::super::wasi::poll::poll::Pollable;
      const _: () = {
        assert!(4 == <Pollable as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Pollable as wasmtime::component::ComponentType>::ALIGN32);
      };
      /// A timestamp in nanoseconds.
      pub type Instant = u64;
      const _: () = {
        assert!(8 == <Instant as wasmtime::component::ComponentType>::SIZE32);
        assert!(8 == <Instant as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub trait Host {
        /// Read the current value of the clock.
        /// 
        /// The clock is monotonic, therefore calling this function repeatedly will
        /// produce a sequence of non-decreasing values.
        fn now(&mut self, ) -> wasmtime::Result<Instant>;
        /// Query the resolution of the clock.
        fn resolution(&mut self, ) -> wasmtime::Result<Instant>;
        /// Create a `pollable` which will resolve once the specified time has been
        /// reached.
        fn subscribe(&mut self, when: Instant,absolute: bool,) -> wasmtime::Result<Pollable>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:clocks/monotonic-clock")?;
        inst.func_wrap("now", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
          let host = get(caller.data_mut());
          let r = host.now();
          Ok((r?,))
        })?;
        inst.func_wrap("resolution", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
          let host = get(caller.data_mut());
          let r = host.resolution();
          Ok((r?,))
        })?;
        inst.func_wrap("subscribe", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (Instant, bool, ) | { 
          let host = get(caller.data_mut());
          let r = host.subscribe(arg0,arg1,);
          Ok((r?,))
        })?;
        Ok(())
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod timezone {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      pub type Datetime = super::super::super::wasi::clocks::wall_clock::Datetime;
      const _: () = {
        assert!(16 == <Datetime as wasmtime::component::ComponentType>::SIZE32);
        assert!(8 == <Datetime as wasmtime::component::ComponentType>::ALIGN32);
      };
      /// A timezone.
      /// 
      /// In timezones that recognize daylight saving time, also known as daylight
      /// time and summer time, the information returned from the functions varies
      /// over time to reflect these adjustments.
      /// 
      /// This [represents a resource](https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources).
      pub type Timezone = u32;
      const _: () = {
        assert!(4 == <Timezone as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Timezone as wasmtime::component::ComponentType>::ALIGN32);
      };
      /// Information useful for displaying the timezone of a specific `datetime`.
      /// 
      /// This information may vary within a single `timezone` to reflect daylight
      /// saving time adjustments.
      #[derive(wasmtime::component::ComponentType)]
      #[derive(wasmtime::component::Lift)]
      #[derive(wasmtime::component::Lower)]
      #[component(record)]
      #[derive(Clone)]
      pub struct TimezoneDisplay {
        /// The number of seconds difference between UTC time and the local
        /// time of the timezone.
        /// 
        /// The returned value will always be less than 86400 which is the
        /// number of seconds in a day (24*60*60).
        /// 
        /// In implementations that do not expose an actual time zone, this
        /// should return 0.
        #[component(name = "utc-offset")]
        pub utc_offset: i32,
        /// The abbreviated name of the timezone to display to a user. The name
        /// `UTC` indicates Coordinated Universal Time. Otherwise, this should
        /// reference local standards for the name of the time zone.
        /// 
        /// In implementations that do not expose an actual time zone, this
        /// should be the string `UTC`.
        /// 
        /// In time zones that do not have an applicable name, a formatted
        /// representation of the UTC offset may be returned, such as `-04:00`.
        #[component(name = "name")]
        pub name: String,
        /// Whether daylight saving time is active.
        /// 
        /// In implementations that do not expose an actual time zone, this
        /// should return false.
        #[component(name = "in-daylight-saving-time")]
        pub in_daylight_saving_time: bool,
      }
      impl core::fmt::Debug for TimezoneDisplay {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("TimezoneDisplay").field("utc-offset", &self.utc_offset).field("name", &self.name).field("in-daylight-saving-time", &self.in_daylight_saving_time).finish()
        }
      }
      const _: () = {
        assert!(16 == <TimezoneDisplay as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <TimezoneDisplay as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub trait Host {
        /// Return information needed to display the given `datetime`. This includes
        /// the UTC offset, the time zone name, and a flag indicating whether
        /// daylight saving time is active.
        /// 
        /// If the timezone cannot be determined for the given `datetime`, return a
        /// `timezone-display` for `UTC` with a `utc-offset` of 0 and no daylight
        /// saving time.
        fn display(&mut self, this: Timezone,when: Datetime,) -> wasmtime::Result<TimezoneDisplay>;
        /// The same as `display`, but only return the UTC offset.
        fn utc_offset(&mut self, this: Timezone,when: Datetime,) -> wasmtime::Result<i32>;
        /// Dispose of the specified input-stream, after which it may no longer
        /// be used.
        fn drop_timezone(&mut self, this: Timezone,) -> wasmtime::Result<()>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:clocks/timezone")?;
        inst.func_wrap("display", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (Timezone, Datetime, ) | { 
          let host = get(caller.data_mut());
          let r = host.display(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("utc-offset", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (Timezone, Datetime, ) | { 
          let host = get(caller.data_mut());
          let r = host.utc_offset(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("drop-timezone", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Timezone, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_timezone(arg0,);
          r
        })?;
        Ok(())
      }
      
    }
    
  }
  pub mod http {
    
    #[allow(clippy::all)]
    pub mod types {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      pub type InputStream = super::super::super::wasi::io::streams::InputStream;
      const _: () = {
        assert!(4 == <InputStream as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <InputStream as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
      const _: () = {
        assert!(4 == <OutputStream as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <OutputStream as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type Pollable = super::super::super::wasi::poll::poll::Pollable;
      const _: () = {
        assert!(4 == <Pollable as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Pollable as wasmtime::component::ComponentType>::ALIGN32);
      };
      #[derive(wasmtime::component::ComponentType)]
      #[derive(wasmtime::component::Lift)]
      #[derive(wasmtime::component::Lower)]
      #[component(variant)]
      #[derive(Clone)]
      pub enum Method{
        #[component(name = "get")] Get,
        #[component(name = "head")] Head,
        #[component(name = "post")] Post,
        #[component(name = "put")] Put,
        #[component(name = "delete")] Delete,
        #[component(name = "connect")] Connect,
        #[component(name = "options")] Options,
        #[component(name = "trace")] Trace,
        #[component(name = "patch")] Patch,
        #[component(name = "other")] Other(String),
      }
      impl core::fmt::Debug for Method {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          match self {
            Method::Get => {
              f.debug_tuple("Method::Get").finish()
            }
            Method::Head => {
              f.debug_tuple("Method::Head").finish()
            }
            Method::Post => {
              f.debug_tuple("Method::Post").finish()
            }
            Method::Put => {
              f.debug_tuple("Method::Put").finish()
            }
            Method::Delete => {
              f.debug_tuple("Method::Delete").finish()
            }
            Method::Connect => {
              f.debug_tuple("Method::Connect").finish()
            }
            Method::Options => {
              f.debug_tuple("Method::Options").finish()
            }
            Method::Trace => {
              f.debug_tuple("Method::Trace").finish()
            }
            Method::Patch => {
              f.debug_tuple("Method::Patch").finish()
            }
            Method::Other(e) => {
              f.debug_tuple("Method::Other").field(e).finish()
            }
          }
        }
      }
      const _: () = {
        assert!(12 == <Method as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Method as wasmtime::component::ComponentType>::ALIGN32);
      };
      #[derive(wasmtime::component::ComponentType)]
      #[derive(wasmtime::component::Lift)]
      #[derive(wasmtime::component::Lower)]
      #[component(variant)]
      #[derive(Clone)]
      pub enum Scheme{
        #[component(name = "HTTP")] Http,
        #[component(name = "HTTPS")] Https,
        #[component(name = "other")] Other(String),
      }
      impl core::fmt::Debug for Scheme {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          match self {
            Scheme::Http => {
              f.debug_tuple("Scheme::Http").finish()
            }
            Scheme::Https => {
              f.debug_tuple("Scheme::Https").finish()
            }
            Scheme::Other(e) => {
              f.debug_tuple("Scheme::Other").field(e).finish()
            }
          }
        }
      }
      const _: () = {
        assert!(12 == <Scheme as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Scheme as wasmtime::component::ComponentType>::ALIGN32);
      };
      #[derive(wasmtime::component::ComponentType)]
      #[derive(wasmtime::component::Lift)]
      #[derive(wasmtime::component::Lower)]
      #[component(variant)]
      #[derive(Clone)]
      pub enum Error{
        #[component(name = "invalid-url")] InvalidUrl(String),
        #[component(name = "timeout-error")] TimeoutError(String),
        #[component(name = "protocol-error")] ProtocolError(String),
        #[component(name = "unexpected-error")] UnexpectedError(String),
      }
      impl core::fmt::Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          match self {
            Error::InvalidUrl(e) => {
              f.debug_tuple("Error::InvalidUrl").field(e).finish()
            }
            Error::TimeoutError(e) => {
              f.debug_tuple("Error::TimeoutError").field(e).finish()
            }
            Error::ProtocolError(e) => {
              f.debug_tuple("Error::ProtocolError").field(e).finish()
            }
            Error::UnexpectedError(e) => {
              f.debug_tuple("Error::UnexpectedError").field(e).finish()
            }
          }
        }
      }
      const _: () = {
        assert!(12 == <Error as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Error as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type Fields = u32;
      const _: () = {
        assert!(4 == <Fields as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Fields as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type Headers = Fields;
      const _: () = {
        assert!(4 == <Headers as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Headers as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type Trailers = Fields;
      const _: () = {
        assert!(4 == <Trailers as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Trailers as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type IncomingStream = InputStream;
      const _: () = {
        assert!(4 == <IncomingStream as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <IncomingStream as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type OutgoingStream = OutputStream;
      const _: () = {
        assert!(4 == <OutgoingStream as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <OutgoingStream as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type IncomingRequest = u32;
      const _: () = {
        assert!(4 == <IncomingRequest as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <IncomingRequest as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type OutgoingRequest = u32;
      const _: () = {
        assert!(4 == <OutgoingRequest as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <OutgoingRequest as wasmtime::component::ComponentType>::ALIGN32);
      };
      #[derive(wasmtime::component::ComponentType)]
      #[derive(wasmtime::component::Lift)]
      #[derive(wasmtime::component::Lower)]
      #[component(record)]
      #[derive(Copy, Clone)]
      pub struct RequestOptions {
        #[component(name = "connect-timeout-ms")]
        pub connect_timeout_ms: Option<u32>,
        #[component(name = "first-byte-timeout-ms")]
        pub first_byte_timeout_ms: Option<u32>,
        #[component(name = "between-bytes-timeout-ms")]
        pub between_bytes_timeout_ms: Option<u32>,
      }
      impl core::fmt::Debug for RequestOptions {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("RequestOptions").field("connect-timeout-ms", &self.connect_timeout_ms).field("first-byte-timeout-ms", &self.first_byte_timeout_ms).field("between-bytes-timeout-ms", &self.between_bytes_timeout_ms).finish()
        }
      }
      const _: () = {
        assert!(24 == <RequestOptions as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <RequestOptions as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type ResponseOutparam = u32;
      const _: () = {
        assert!(4 == <ResponseOutparam as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <ResponseOutparam as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type StatusCode = u16;
      const _: () = {
        assert!(2 == <StatusCode as wasmtime::component::ComponentType>::SIZE32);
        assert!(2 == <StatusCode as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type IncomingResponse = u32;
      const _: () = {
        assert!(4 == <IncomingResponse as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <IncomingResponse as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type OutgoingResponse = u32;
      const _: () = {
        assert!(4 == <OutgoingResponse as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <OutgoingResponse as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type FutureIncomingResponse = u32;
      const _: () = {
        assert!(4 == <FutureIncomingResponse as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <FutureIncomingResponse as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub trait Host {
        fn drop_fields(&mut self, fields: Fields,) -> wasmtime::Result<()>;
        fn new_fields(&mut self, entries: Vec<(String,String,)>,) -> wasmtime::Result<Fields>;
        fn fields_get(&mut self, fields: Fields,name: String,) -> wasmtime::Result<Vec<Vec<u8>>>;
        fn fields_set(&mut self, fields: Fields,name: String,value: Vec<Vec<u8>>,) -> wasmtime::Result<()>;
        fn fields_delete(&mut self, fields: Fields,name: String,) -> wasmtime::Result<()>;
        fn fields_append(&mut self, fields: Fields,name: String,value: Vec<u8>,) -> wasmtime::Result<()>;
        fn fields_entries(&mut self, fields: Fields,) -> wasmtime::Result<Vec<(String,Vec<u8>,)>>;
        fn fields_clone(&mut self, fields: Fields,) -> wasmtime::Result<Fields>;
        fn finish_incoming_stream(&mut self, s: IncomingStream,) -> wasmtime::Result<Option<Trailers>>;
        fn finish_outgoing_stream(&mut self, s: OutgoingStream,trailers: Option<Trailers>,) -> wasmtime::Result<()>;
        fn drop_incoming_request(&mut self, request: IncomingRequest,) -> wasmtime::Result<()>;
        fn drop_outgoing_request(&mut self, request: OutgoingRequest,) -> wasmtime::Result<()>;
        fn incoming_request_method(&mut self, request: IncomingRequest,) -> wasmtime::Result<Method>;
        fn incoming_request_path_with_query(&mut self, request: IncomingRequest,) -> wasmtime::Result<Option<String>>;
        fn incoming_request_scheme(&mut self, request: IncomingRequest,) -> wasmtime::Result<Option<Scheme>>;
        fn incoming_request_authority(&mut self, request: IncomingRequest,) -> wasmtime::Result<Option<String>>;
        fn incoming_request_headers(&mut self, request: IncomingRequest,) -> wasmtime::Result<Headers>;
        fn incoming_request_consume(&mut self, request: IncomingRequest,) -> wasmtime::Result<Result<IncomingStream,()>>;
        fn new_outgoing_request(&mut self, method: Method,path_with_query: Option<String>,scheme: Option<Scheme>,authority: Option<String>,headers: Headers,) -> wasmtime::Result<OutgoingRequest>;
        fn outgoing_request_write(&mut self, request: OutgoingRequest,) -> wasmtime::Result<Result<OutgoingStream,()>>;
        fn drop_response_outparam(&mut self, response: ResponseOutparam,) -> wasmtime::Result<()>;
        fn set_response_outparam(&mut self, param: ResponseOutparam,response: Result<OutgoingResponse,Error>,) -> wasmtime::Result<Result<(),()>>;
        fn drop_incoming_response(&mut self, response: IncomingResponse,) -> wasmtime::Result<()>;
        fn drop_outgoing_response(&mut self, response: OutgoingResponse,) -> wasmtime::Result<()>;
        fn incoming_response_status(&mut self, response: IncomingResponse,) -> wasmtime::Result<StatusCode>;
        fn incoming_response_headers(&mut self, response: IncomingResponse,) -> wasmtime::Result<Headers>;
        fn incoming_response_consume(&mut self, response: IncomingResponse,) -> wasmtime::Result<Result<IncomingStream,()>>;
        fn new_outgoing_response(&mut self, status_code: StatusCode,headers: Headers,) -> wasmtime::Result<OutgoingResponse>;
        fn outgoing_response_write(&mut self, response: OutgoingResponse,) -> wasmtime::Result<Result<OutgoingStream,()>>;
        fn drop_future_incoming_response(&mut self, f: FutureIncomingResponse,) -> wasmtime::Result<()>;
        fn future_incoming_response_get(&mut self, f: FutureIncomingResponse,) -> wasmtime::Result<Option<Result<IncomingResponse,Error>>>;
        fn listen_to_future_incoming_response(&mut self, f: FutureIncomingResponse,) -> wasmtime::Result<Pollable>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:http/types")?;
        inst.func_wrap("drop-fields", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Fields, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_fields(arg0,);
          r
        })?;
        inst.func_wrap("new-fields", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Vec<(String,String,)>, ) | { 
          let host = get(caller.data_mut());
          let r = host.new_fields(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("fields-get", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (Fields, String, ) | { 
          let host = get(caller.data_mut());
          let r = host.fields_get(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("fields-set", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,arg2,) : (Fields, String, Vec<Vec<u8>>, ) | { 
          let host = get(caller.data_mut());
          let r = host.fields_set(arg0,arg1,arg2,);
          r
        })?;
        inst.func_wrap("fields-delete", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (Fields, String, ) | { 
          let host = get(caller.data_mut());
          let r = host.fields_delete(arg0,arg1,);
          r
        })?;
        inst.func_wrap("fields-append", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,arg2,) : (Fields, String, Vec<u8>, ) | { 
          let host = get(caller.data_mut());
          let r = host.fields_append(arg0,arg1,arg2,);
          r
        })?;
        inst.func_wrap("fields-entries", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Fields, ) | { 
          let host = get(caller.data_mut());
          let r = host.fields_entries(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("fields-clone", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Fields, ) | { 
          let host = get(caller.data_mut());
          let r = host.fields_clone(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("finish-incoming-stream", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingStream, ) | { 
          let host = get(caller.data_mut());
          let r = host.finish_incoming_stream(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("finish-outgoing-stream", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (OutgoingStream, Option<Trailers>, ) | { 
          let host = get(caller.data_mut());
          let r = host.finish_outgoing_stream(arg0,arg1,);
          r
        })?;
        inst.func_wrap("drop-incoming-request", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingRequest, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_incoming_request(arg0,);
          r
        })?;
        inst.func_wrap("drop-outgoing-request", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (OutgoingRequest, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_outgoing_request(arg0,);
          r
        })?;
        inst.func_wrap("incoming-request-method", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingRequest, ) | { 
          let host = get(caller.data_mut());
          let r = host.incoming_request_method(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("incoming-request-path-with-query", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingRequest, ) | { 
          let host = get(caller.data_mut());
          let r = host.incoming_request_path_with_query(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("incoming-request-scheme", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingRequest, ) | { 
          let host = get(caller.data_mut());
          let r = host.incoming_request_scheme(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("incoming-request-authority", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingRequest, ) | { 
          let host = get(caller.data_mut());
          let r = host.incoming_request_authority(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("incoming-request-headers", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingRequest, ) | { 
          let host = get(caller.data_mut());
          let r = host.incoming_request_headers(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("incoming-request-consume", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingRequest, ) | { 
          let host = get(caller.data_mut());
          let r = host.incoming_request_consume(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("new-outgoing-request", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,arg2,arg3,arg4,) : (Method, Option<String>, Option<Scheme>, Option<String>, Headers, ) | { 
          let host = get(caller.data_mut());
          let r = host.new_outgoing_request(arg0,arg1,arg2,arg3,arg4,);
          Ok((r?,))
        })?;
        inst.func_wrap("outgoing-request-write", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (OutgoingRequest, ) | { 
          let host = get(caller.data_mut());
          let r = host.outgoing_request_write(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("drop-response-outparam", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (ResponseOutparam, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_response_outparam(arg0,);
          r
        })?;
        inst.func_wrap("set-response-outparam", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (ResponseOutparam, Result<OutgoingResponse,Error>, ) | { 
          let host = get(caller.data_mut());
          let r = host.set_response_outparam(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("drop-incoming-response", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingResponse, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_incoming_response(arg0,);
          r
        })?;
        inst.func_wrap("drop-outgoing-response", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (OutgoingResponse, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_outgoing_response(arg0,);
          r
        })?;
        inst.func_wrap("incoming-response-status", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingResponse, ) | { 
          let host = get(caller.data_mut());
          let r = host.incoming_response_status(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("incoming-response-headers", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingResponse, ) | { 
          let host = get(caller.data_mut());
          let r = host.incoming_response_headers(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("incoming-response-consume", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (IncomingResponse, ) | { 
          let host = get(caller.data_mut());
          let r = host.incoming_response_consume(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("new-outgoing-response", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (StatusCode, Headers, ) | { 
          let host = get(caller.data_mut());
          let r = host.new_outgoing_response(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("outgoing-response-write", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (OutgoingResponse, ) | { 
          let host = get(caller.data_mut());
          let r = host.outgoing_response_write(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("drop-future-incoming-response", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (FutureIncomingResponse, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_future_incoming_response(arg0,);
          r
        })?;
        inst.func_wrap("future-incoming-response-get", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (FutureIncomingResponse, ) | { 
          let host = get(caller.data_mut());
          let r = host.future_incoming_response_get(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("listen-to-future-incoming-response", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (FutureIncomingResponse, ) | { 
          let host = get(caller.data_mut());
          let r = host.listen_to_future_incoming_response(arg0,);
          Ok((r?,))
        })?;
        Ok(())
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod outgoing_handler {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      pub type OutgoingRequest = super::super::super::wasi::http::types::OutgoingRequest;
      const _: () = {
        assert!(4 == <OutgoingRequest as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <OutgoingRequest as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type RequestOptions = super::super::super::wasi::http::types::RequestOptions;
      const _: () = {
        assert!(24 == <RequestOptions as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <RequestOptions as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub type FutureIncomingResponse = super::super::super::wasi::http::types::FutureIncomingResponse;
      const _: () = {
        assert!(4 == <FutureIncomingResponse as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <FutureIncomingResponse as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub trait Host {
        fn handle(&mut self, request: OutgoingRequest,options: Option<RequestOptions>,) -> wasmtime::Result<FutureIncomingResponse>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:http/outgoing-handler")?;
        inst.func_wrap("handle", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (OutgoingRequest, Option<RequestOptions>, ) | { 
          let host = get(caller.data_mut());
          let r = host.handle(arg0,arg1,);
          Ok((r?,))
        })?;
        Ok(())
      }
      
    }
    
  }
  pub mod io {
    
    #[allow(clippy::all)]
    pub mod streams {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      pub type Pollable = super::super::super::wasi::poll::poll::Pollable;
      const _: () = {
        assert!(4 == <Pollable as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <Pollable as wasmtime::component::ComponentType>::ALIGN32);
      };
      /// An error type returned from a stream operation. Currently this
      /// doesn't provide any additional information.
      #[derive(wasmtime::component::ComponentType)]
      #[derive(wasmtime::component::Lift)]
      #[derive(wasmtime::component::Lower)]
      #[component(record)]
      #[derive(Copy, Clone)]
      pub struct StreamError {
      }
      impl core::fmt::Debug for StreamError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("StreamError").finish()
        }
      }
      impl core::fmt::Display for StreamError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          write!(f, "{:?}", self)
        }
      }
      impl std::error::Error for StreamError{}
      const _: () = {
        assert!(0 == <StreamError as wasmtime::component::ComponentType>::SIZE32);
        assert!(1 == <StreamError as wasmtime::component::ComponentType>::ALIGN32);
      };
      /// An input bytestream. In the future, this will be replaced by handle
      /// types.
      /// 
      /// This conceptually represents a `stream<u8, _>`. It's temporary
      /// scaffolding until component-model's async features are ready.
      /// 
      /// `input-stream`s are *non-blocking* to the extent practical on underlying
      /// platforms. I/O operations always return promptly; if fewer bytes are
      /// promptly available than requested, they return the number of bytes promptly
      /// available, which could even be zero. To wait for data to be available,
      /// use the `subscribe-to-input-stream` function to obtain a `pollable` which
      /// can be polled for using `wasi_poll`.
      /// 
      /// And at present, it is a `u32` instead of being an actual handle, until
      /// the wit-bindgen implementation of handles and resources is ready.
      /// 
      /// This [represents a resource](https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources).
      pub type InputStream = u32;
      const _: () = {
        assert!(4 == <InputStream as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <InputStream as wasmtime::component::ComponentType>::ALIGN32);
      };
      /// An output bytestream. In the future, this will be replaced by handle
      /// types.
      /// 
      /// This conceptually represents a `stream<u8, _>`. It's temporary
      /// scaffolding until component-model's async features are ready.
      /// 
      /// `output-stream`s are *non-blocking* to the extent practical on
      /// underlying platforms. Except where specified otherwise, I/O operations also
      /// always return promptly, after the number of bytes that can be written
      /// promptly, which could even be zero. To wait for the stream to be ready to
      /// accept data, the `subscribe-to-output-stream` function to obtain a
      /// `pollable` which can be polled for using `wasi_poll`.
      /// 
      /// And at present, it is a `u32` instead of being an actual handle, until
      /// the wit-bindgen implementation of handles and resources is ready.
      /// 
      /// This [represents a resource](https://github.com/WebAssembly/WASI/blob/main/docs/WitInWasi.md#Resources).
      pub type OutputStream = u32;
      const _: () = {
        assert!(4 == <OutputStream as wasmtime::component::ComponentType>::SIZE32);
        assert!(4 == <OutputStream as wasmtime::component::ComponentType>::ALIGN32);
      };
      pub trait Host {
        /// Read bytes from a stream.
        /// 
        /// This function returns a list of bytes containing the data that was
        /// read, along with a bool which, when true, indicates that the end of the
        /// stream was reached. The returned list will contain up to `len` bytes; it
        /// may return fewer than requested, but not more.
        /// 
        /// Once a stream has reached the end, subsequent calls to read or
        /// `skip` will always report end-of-stream rather than producing more
        /// data.
        /// 
        /// If `len` is 0, it represents a request to read 0 bytes, which should
        /// always succeed, assuming the stream hasn't reached its end yet, and
        /// return an empty list.
        /// 
        /// The len here is a `u64`, but some callees may not be able to allocate
        /// a buffer as large as that would imply.
        /// FIXME: describe what happens if allocation fails.
        fn read(&mut self, this: InputStream,len: u64,) -> wasmtime::Result<Result<(Vec<u8>,bool,),StreamError>>;
        /// Read bytes from a stream, with blocking.
        /// 
        /// This is similar to `read`, except that it blocks until at least one
        /// byte can be read.
        fn blocking_read(&mut self, this: InputStream,len: u64,) -> wasmtime::Result<Result<(Vec<u8>,bool,),StreamError>>;
        /// Skip bytes from a stream.
        /// 
        /// This is similar to the `read` function, but avoids copying the
        /// bytes into the instance.
        /// 
        /// Once a stream has reached the end, subsequent calls to read or
        /// `skip` will always report end-of-stream rather than producing more
        /// data.
        /// 
        /// This function returns the number of bytes skipped, along with a bool
        /// indicating whether the end of the stream was reached. The returned
        /// value will be at most `len`; it may be less.
        fn skip(&mut self, this: InputStream,len: u64,) -> wasmtime::Result<Result<(u64,bool,),StreamError>>;
        /// Skip bytes from a stream, with blocking.
        /// 
        /// This is similar to `skip`, except that it blocks until at least one
        /// byte can be consumed.
        fn blocking_skip(&mut self, this: InputStream,len: u64,) -> wasmtime::Result<Result<(u64,bool,),StreamError>>;
        /// Create a `pollable` which will resolve once either the specified stream
        /// has bytes available to read or the other end of the stream has been
        /// closed.
        fn subscribe_to_input_stream(&mut self, this: InputStream,) -> wasmtime::Result<Pollable>;
        /// Dispose of the specified `input-stream`, after which it may no longer
        /// be used.
        fn drop_input_stream(&mut self, this: InputStream,) -> wasmtime::Result<()>;
        /// Write bytes to a stream.
        /// 
        /// This function returns a `u64` indicating the number of bytes from
        /// `buf` that were written; it may be less than the full list.
        fn write(&mut self, this: OutputStream,buf: Vec<u8>,) -> wasmtime::Result<Result<u64,StreamError>>;
        /// Write bytes to a stream, with blocking.
        /// 
        /// This is similar to `write`, except that it blocks until at least one
        /// byte can be written.
        fn blocking_write(&mut self, this: OutputStream,buf: Vec<u8>,) -> wasmtime::Result<Result<u64,StreamError>>;
        /// Write multiple zero bytes to a stream.
        /// 
        /// This function returns a `u64` indicating the number of zero bytes
        /// that were written; it may be less than `len`.
        fn write_zeroes(&mut self, this: OutputStream,len: u64,) -> wasmtime::Result<Result<u64,StreamError>>;
        /// Write multiple zero bytes to a stream, with blocking.
        /// 
        /// This is similar to `write-zeroes`, except that it blocks until at least
        /// one byte can be written.
        fn blocking_write_zeroes(&mut self, this: OutputStream,len: u64,) -> wasmtime::Result<Result<u64,StreamError>>;
        /// Read from one stream and write to another.
        /// 
        /// This function returns the number of bytes transferred; it may be less
        /// than `len`.
        /// 
        /// Unlike other I/O functions, this function blocks until all the data
        /// read from the input stream has been written to the output stream.
        fn splice(&mut self, this: OutputStream,src: InputStream,len: u64,) -> wasmtime::Result<Result<(u64,bool,),StreamError>>;
        /// Read from one stream and write to another, with blocking.
        /// 
        /// This is similar to `splice`, except that it blocks until at least
        /// one byte can be read.
        fn blocking_splice(&mut self, this: OutputStream,src: InputStream,len: u64,) -> wasmtime::Result<Result<(u64,bool,),StreamError>>;
        /// Forward the entire contents of an input stream to an output stream.
        /// 
        /// This function repeatedly reads from the input stream and writes
        /// the data to the output stream, until the end of the input stream
        /// is reached, or an error is encountered.
        /// 
        /// Unlike other I/O functions, this function blocks until the end
        /// of the input stream is seen and all the data has been written to
        /// the output stream.
        /// 
        /// This function returns the number of bytes transferred.
        fn forward(&mut self, this: OutputStream,src: InputStream,) -> wasmtime::Result<Result<u64,StreamError>>;
        /// Create a `pollable` which will resolve once either the specified stream
        /// is ready to accept bytes or the other end of the stream has been closed.
        fn subscribe_to_output_stream(&mut self, this: OutputStream,) -> wasmtime::Result<Pollable>;
        /// Dispose of the specified `output-stream`, after which it may no longer
        /// be used.
        fn drop_output_stream(&mut self, this: OutputStream,) -> wasmtime::Result<()>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:io/streams")?;
        inst.func_wrap("read", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (InputStream, u64, ) | { 
          let host = get(caller.data_mut());
          let r = host.read(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("blocking-read", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (InputStream, u64, ) | { 
          let host = get(caller.data_mut());
          let r = host.blocking_read(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("skip", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (InputStream, u64, ) | { 
          let host = get(caller.data_mut());
          let r = host.skip(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("blocking-skip", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (InputStream, u64, ) | { 
          let host = get(caller.data_mut());
          let r = host.blocking_skip(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("subscribe-to-input-stream", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (InputStream, ) | { 
          let host = get(caller.data_mut());
          let r = host.subscribe_to_input_stream(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("drop-input-stream", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (InputStream, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_input_stream(arg0,);
          r
        })?;
        inst.func_wrap("write", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (OutputStream, Vec<u8>, ) | { 
          let host = get(caller.data_mut());
          let r = host.write(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("blocking-write", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (OutputStream, Vec<u8>, ) | { 
          let host = get(caller.data_mut());
          let r = host.blocking_write(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("write-zeroes", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (OutputStream, u64, ) | { 
          let host = get(caller.data_mut());
          let r = host.write_zeroes(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("blocking-write-zeroes", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (OutputStream, u64, ) | { 
          let host = get(caller.data_mut());
          let r = host.blocking_write_zeroes(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("splice", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,arg2,) : (OutputStream, InputStream, u64, ) | { 
          let host = get(caller.data_mut());
          let r = host.splice(arg0,arg1,arg2,);
          Ok((r?,))
        })?;
        inst.func_wrap("blocking-splice", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,arg2,) : (OutputStream, InputStream, u64, ) | { 
          let host = get(caller.data_mut());
          let r = host.blocking_splice(arg0,arg1,arg2,);
          Ok((r?,))
        })?;
        inst.func_wrap("forward", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (OutputStream, InputStream, ) | { 
          let host = get(caller.data_mut());
          let r = host.forward(arg0,arg1,);
          Ok((r?,))
        })?;
        inst.func_wrap("subscribe-to-output-stream", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (OutputStream, ) | { 
          let host = get(caller.data_mut());
          let r = host.subscribe_to_output_stream(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("drop-output-stream", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (OutputStream, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_output_stream(arg0,);
          r
        })?;
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
      pub trait Host {
        /// Dispose of the specified `pollable`, after which it may no longer
        /// be used.
        fn drop_pollable(&mut self, this: Pollable,) -> wasmtime::Result<()>;
        /// Poll for completion on a set of pollables.
        /// 
        /// The "oneoff" in the name refers to the fact that this function must do a
        /// linear scan through the entire list of subscriptions, which may be
        /// inefficient if the number is large and the same subscriptions are used
        /// many times. In the future, this is expected to be obsoleted by the
        /// component model async proposal, which will include a scalable waiting
        /// facility.
        /// 
        /// Note that the return type would ideally be `list<bool>`, but that would
        /// be more difficult to polyfill given the current state of `wit-bindgen`.
        /// See <https://github.com/bytecodealliance/preview2-prototyping/pull/11#issuecomment-1329873061>
        /// for details.  For now, we use zero to mean "not ready" and non-zero to
        /// mean "ready".
        fn poll_oneoff(&mut self, in_: Vec<Pollable>,) -> wasmtime::Result<Vec<u8>>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:poll/poll")?;
        inst.func_wrap("drop-pollable", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Pollable, ) | { 
          let host = get(caller.data_mut());
          let r = host.drop_pollable(arg0,);
          r
        })?;
        inst.func_wrap("poll-oneoff", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (Vec<Pollable>, ) | { 
          let host = get(caller.data_mut());
          let r = host.poll_oneoff(arg0,);
          Ok((r?,))
        })?;
        Ok(())
      }
      
    }
    
  }
  pub mod random {
    
    #[allow(clippy::all)]
    pub mod random {
      #[allow(unused_imports)]
      use wasmtime::component::__internal::anyhow;
      
      pub trait Host {
        /// Return `len` cryptographically-secure pseudo-random bytes.
        /// 
        /// This function must produce data from an adequately seeded
        /// cryptographically-secure pseudo-random number generator (CSPRNG), so it
        /// must not block, from the perspective of the calling program, and the
        /// returned data is always unpredictable.
        /// 
        /// This function must always return fresh pseudo-random data. Deterministic
        /// environments must omit this function, rather than implementing it with
        /// deterministic data.
        fn get_random_bytes(&mut self, len: u64,) -> wasmtime::Result<Vec<u8>>;
        /// Return a cryptographically-secure pseudo-random `u64` value.
        /// 
        /// This function returns the same type of pseudo-random data as
        /// `get-random-bytes`, represented as a `u64`.
        fn get_random_u64(&mut self, ) -> wasmtime::Result<u64>;
      }
      
      pub fn add_to_linker<T, U>(
      linker: &mut wasmtime::component::Linker<T>,
      get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
      ) -> wasmtime::Result<()>
      where U: Host,
      {
        
        let mut inst = linker.instance("wasi:random/random")?;
        inst.func_wrap("get-random-bytes", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (u64, ) | { 
          let host = get(caller.data_mut());
          let r = host.get_random_bytes(arg0,);
          Ok((r?,))
        })?;
        inst.func_wrap("get-random-u64", move |mut caller: wasmtime::StoreContextMut<'_, T>, () : () | { 
          let host = get(caller.data_mut());
          let r = host.get_random_u64();
          Ok((r?,))
        })?;
        Ok(())
      }
      
    }
    
  }
}
pub mod exports {
  pub mod wasi {
    pub mod http {
      
      #[allow(clippy::all)]
      pub mod incoming_handler {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::anyhow;
        
        pub type IncomingRequest = super::super::super::super::wasi::http::types::IncomingRequest;
        const _: () = {
          assert!(4 == <IncomingRequest as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <IncomingRequest as wasmtime::component::ComponentType>::ALIGN32);
        };
        pub type ResponseOutparam = super::super::super::super::wasi::http::types::ResponseOutparam;
        const _: () = {
          assert!(4 == <ResponseOutparam as wasmtime::component::ComponentType>::SIZE32);
          assert!(4 == <ResponseOutparam as wasmtime::component::ComponentType>::ALIGN32);
        };
        pub struct IncomingHandler {
          handle: wasmtime::component::Func,
        }
        impl IncomingHandler {
          
          pub fn new(
          __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
          ) -> wasmtime::Result<IncomingHandler> {
            let handle = *__exports.typed_func::<(IncomingRequest, ResponseOutparam, ), ()>("handle")?.func();
            Ok(IncomingHandler {
              handle,
            })
          }
          pub  fn call_handle<S: wasmtime::AsContextMut>(&self, mut store: S, arg0: IncomingRequest,arg1: ResponseOutparam,) -> wasmtime::Result<()> {
            let callee = unsafe {
              wasmtime::component::TypedFunc::<(IncomingRequest, ResponseOutparam, ), ()>::new_unchecked(self.handle)
            };
            let () = callee.call(store.as_context_mut(), (arg0, arg1, ))?;
            callee.post_return(store.as_context_mut())?;
            Ok(())
          }
        }
        
      }
      
    }
  }
}
