#![feature(macro_rules)]

extern crate mongoose;
use std::libc;
use libc::types::common::c95::c_void;

/// C's void, easy to use.
macro_rules! void( () => (&mut 0 as *mut _ as *mut c_void); )

/// Utility to make using mongoose easier. Should eventually contain
/// all of the `unsafe` blocks used for mongoose.
mod mg {
  use mongoose;
  use libc::types::common::c95::c_void;

  pub trait Server {
    fn setOption(self, opt: &str, val: &str);
    fn pollServer(self, wait: i32);
    fn destroyServer(self);
  }

  pub fn createServer(param: *mut c_void, handler: mongoose::mg_handler_t) -> *mut mongoose::Struct_mg_server {
    unsafe {
      mongoose::mg_create_server(param, handler)
    }
  }

  impl Server for *mut mongoose::Struct_mg_server {
    fn setOption(self, opt: &str, val: &str) {
      let c_opt = opt.to_c_str();
      let c_val = val.to_c_str();

      c_opt.with_ref(|b_opt| {
        c_val.with_ref(|b_val| {
          unsafe {
            mongoose::mg_set_option(self, b_opt, b_val);
          }
        })
      })
    }

    fn pollServer(self, wait: i32) {
      unsafe {
        mongoose::mg_poll_server(self, wait);
      }
    }

    fn destroyServer(self) {
      unsafe {
        mongoose::mg_destroy_server(self as *mut *mut mongoose::Struct_mg_server);
      }
    }
  }
}


fn main() {
  use mg::Server;
  let server = mg::createServer(void!(), None);

  server.setOption("document_root", ".");
  server.setOption("listening_port", "8080");

  println!("Server listening on :8080");
  loop {
    server.pollServer(1000);
  }

  server.destroyServer();
}
