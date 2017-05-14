extern crate nix;

use nix::sys::socket::connect;

struct WLDisplay {
  queues: Vec<WLEventQueue>,
  socket: UnixStream,
}

impl WLDisplay {
  pub fn create_queue(&mut self) -> &WLEventQueue {
    self.queues.push(WLEventQueue::new());
    return &self.queues[self.queues.len()];
  }

  pub fn connect_to_fd(fd: RawFd) -> Result<WLDisplay, &'static str> {
    Ok(
      WLDisplay {
      queues: Vec::<WLEventQueue>::new(),
      socket: unsafe {UnixStream::from_raw_fd(fd)} ,
      }
    )
  }

  pub fn connect(name: Option<&str>) -> Result<WLDisplay, &str> {
    unimplemented!();
    //not sure about this
  }

  pub fn disconnect(&mut self) {
    unimplemented!();
  }

  pub fn get_fd(&self) -> RawFd {
    self.socket.as_raw_fd()
  }
}

struct WLEventQueue {
}

impl WLEventQueue {
  pub fn new() -> WLEventQueue {
    unimplemented!();
  }
}
