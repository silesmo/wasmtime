use crate::preview2::{Table, TableError};
use anyhow::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StreamState {
    Open,
    Closed,
}

impl StreamState {
    pub fn is_closed(&self) -> bool {
        *self == Self::Closed
    }
}

/// An input bytestream.
///
/// This is "pseudo" because the real streams will be a type in wit, and
/// built into the wit bindings, and will support async and type parameters.
/// This pseudo-stream abstraction is synchronous and only supports bytes.
#[async_trait::async_trait]
pub trait HostInputStream: Send + Sync {
    /// Read bytes. On success, returns a pair holding the number of bytes read
    /// and a flag indicating whether the end of the stream was reached.
    fn read(&mut self, buf: &mut [u8]) -> Result<(u64, StreamState), Error>;

    /// Vectored-I/O form of `read`.
    fn read_vectored<'a>(
        &mut self,
        bufs: &mut [std::io::IoSliceMut<'a>],
    ) -> Result<(u64, StreamState), Error> {
        if bufs.len() > 0 {
            self.read(bufs.get_mut(0).unwrap())
        } else {
            self.read(&mut [])
        }
    }

    /// Test whether vectored I/O reads are known to be optimized in the
    /// underlying implementation.
    fn is_read_vectored(&self) -> bool {
        false
    }

    /// Read bytes from a stream and discard them.
    fn skip(&mut self, nelem: u64) -> Result<(u64, StreamState), Error> {
        let mut nread = 0;
        let mut state = StreamState::Open;

        // TODO: Optimize by reading more than one byte at a time.
        for _ in 0..nelem {
            let (num, read_state) = self.read(&mut [0])?;
            nread += num;
            if read_state.is_closed() {
                state = read_state;
                break;
            }
        }

        Ok((nread, state))
    }

    /// An async method to check read readiness.
    async fn ready(&mut self) -> Result<(), Error>;
}

/// An output bytestream.
///
/// This is "pseudo" because the real streams will be a type in wit, and
/// built into the wit bindings, and will support async and type parameters.
/// This pseudo-stream abstraction is synchronous and only supports bytes.
#[async_trait::async_trait]
pub trait HostOutputStream: Send + Sync {
    /// Write bytes. On success, returns the number of bytes written.
    fn write(&mut self, _buf: &[u8]) -> Result<u64, Error>;

    /// Vectored-I/O form of `write`.
    fn write_vectored<'a>(&mut self, bufs: &[std::io::IoSlice<'a>]) -> Result<u64, Error> {
        if bufs.len() > 0 {
            self.write(bufs.get(0).unwrap())
        } else {
            Ok(0)
        }
    }

    /// Test whether vectored I/O writes are known to be optimized in the
    /// underlying implementation.
    fn is_write_vectored(&self) -> bool {
        false
    }

    /// Transfer bytes directly from an input stream to an output stream.
    fn splice(
        &mut self,
        src: &mut dyn HostInputStream,
        nelem: u64,
    ) -> Result<(u64, StreamState), Error> {
        let mut nspliced = 0;
        let mut state = StreamState::Open;

        // TODO: Optimize by splicing more than one byte at a time.
        for _ in 0..nelem {
            let mut buf = [0u8];
            let (num, read_state) = src.read(&mut buf)?;
            self.write(&buf)?;
            nspliced += num;
            if read_state.is_closed() {
                state = read_state;
                break;
            }
        }

        Ok((nspliced, state))
    }

    /// Repeatedly write a byte to a stream.
    fn write_zeroes(&mut self, nelem: u64) -> Result<u64, Error> {
        let mut nwritten = 0;

        // TODO: Optimize by writing more than one byte at a time.
        for _ in 0..nelem {
            let num = self.write(&[0])?;
            if num == 0 {
                break;
            }
            nwritten += num;
        }

        Ok(nwritten)
    }

    /// An async method to check write readiness.
    async fn ready(&mut self) -> Result<(), Error>;
}

pub trait TableStreamExt {
    fn push_input_stream(&mut self, istream: Box<dyn HostInputStream>) -> Result<u32, TableError>;
    fn get_input_stream(&self, fd: u32) -> Result<&dyn HostInputStream, TableError>;
    fn get_input_stream_mut(
        &mut self,
        fd: u32,
    ) -> Result<&mut Box<dyn HostInputStream>, TableError>;

    fn push_output_stream(&mut self, ostream: Box<dyn HostOutputStream>)
        -> Result<u32, TableError>;
    fn get_output_stream(&self, fd: u32) -> Result<&dyn HostOutputStream, TableError>;
    fn get_output_stream_mut(
        &mut self,
        fd: u32,
    ) -> Result<&mut Box<dyn HostOutputStream>, TableError>;
}
impl TableStreamExt for Table {
    fn push_input_stream(&mut self, istream: Box<dyn HostInputStream>) -> Result<u32, TableError> {
        self.push(Box::new(istream))
    }
    fn get_input_stream(&self, fd: u32) -> Result<&dyn HostInputStream, TableError> {
        self.get::<Box<dyn HostInputStream>>(fd).map(|f| f.as_ref())
    }
    fn get_input_stream_mut(
        &mut self,
        fd: u32,
    ) -> Result<&mut Box<dyn HostInputStream>, TableError> {
        self.get_mut::<Box<dyn HostInputStream>>(fd)
    }

    fn push_output_stream(
        &mut self,
        ostream: Box<dyn HostOutputStream>,
    ) -> Result<u32, TableError> {
        self.push(Box::new(ostream))
    }
    fn get_output_stream(&self, fd: u32) -> Result<&dyn HostOutputStream, TableError> {
        self.get::<Box<dyn HostOutputStream>>(fd)
            .map(|f| f.as_ref())
    }
    fn get_output_stream_mut(
        &mut self,
        fd: u32,
    ) -> Result<&mut Box<dyn HostOutputStream>, TableError> {
        self.get_mut::<Box<dyn HostOutputStream>>(fd)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::preview2::pipe::{ReadPipe, WritePipe};
    #[test]
    fn input_stream_in_table() {
        let empty_pipe = ReadPipe::new(std::io::empty());
        let mut table = Table::new();
        let ix = table.push_input_stream(Box::new(empty_pipe)).unwrap();
        let _ = table.get_input_stream(ix).unwrap();
        let _ = table.get_input_stream_mut(ix).unwrap();
    }

    #[test]
    fn output_stream_in_table() {
        let dev_null = WritePipe::new(std::io::sink());
        let mut table = Table::new();
        let ix = table.push_output_stream(Box::new(dev_null)).unwrap();
        let _ = table.get_output_stream(ix).unwrap();
        let _ = table.get_output_stream_mut(ix).unwrap();
    }
}
