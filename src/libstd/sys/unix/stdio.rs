use io;
use libc;
use sys::fd::FileDesc;

pub struct Stdin(());
pub struct Stdout(());
pub struct Stderr(());

impl Stdin {
    pub fn new() -> io::Result<Stdin> { Ok(Stdin(())) }

    pub fn read(&self, data: &mut [u8]) -> io::Result<usize> {
        let fd = FileDesc::new(libc::STDIN_FILENO);
        let ret = fd.read(data);
        fd.into_raw(); // do not close this FD
        ret
    }
}

impl Stdout {
    pub fn new() -> io::Result<Stdout> { Ok(Stdout(())) }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        let fd = FileDesc::new(libc::STDOUT_FILENO);
        let ret = fd.write(data);
        fd.into_raw(); // do not close this FD
        ret
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> { Ok(Stderr(())) }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        let fd = FileDesc::new(libc::STDERR_FILENO);
        let ret = fd.write(data);
        fd.into_raw(); // do not close this FD
        ret
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

pub fn is_ebadf(err: &io::Error) -> bool {
    err.raw_os_error() == Some(libc::EBADF as i32)
}

pub const STDIN_BUF_SIZE: usize = ::sys_common::io::DEFAULT_BUF_SIZE;

pub fn panic_output() -> Option<impl io::Write> {
    io::stderr_raw().ok()
}
