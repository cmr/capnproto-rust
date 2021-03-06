/*
 * Copyright (c) 2013-2014, David Renshaw (dwrenshaw@gmail.com)
 *
 * See the LICENSE file in the capnproto-rust root directory.
 */

#![feature(globs)]
#![feature(macro_rules)]
#![feature(phase)]
#![allow(experimental)]
#![feature(unsafe_destructor)]

#![crate_name="capnp"]
#![crate_type = "lib"]

// import logging macros
#[phase(plugin, link)] extern crate log;
extern crate libc;

#[cfg(test)]
extern crate debug;

// reexports
pub use blob::{text, data};
pub use common::{MessageSize};
pub use list::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};
pub use message::{MessageBuilder, BuilderOptions, MessageReader, ReaderOptions};
pub use message::MallocMessageBuilder;
pub use serialize::OwnedSpaceMessageReader;

pub mod any_pointer;
pub mod arena;
pub mod blob;
pub mod capability;
pub mod common;
pub mod endian;
pub mod io;
pub mod layout;
pub mod list;
pub mod mask;
pub mod message;
pub mod serialize;
pub mod serialize_packed;


#[cfg(test)]
pub mod layout_test;
#[cfg(test)]
pub mod serialize_packed_test;




