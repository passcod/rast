#![crate_id = "memstream#0.1"]

//! # MemStream
//!
//! Like a [`MemWriter`] and [`MemReader`], but both
//! at the same time. It is backed by a `Deque<u8>`.
//!
//! This crate provides `MemStream`s backed by all `Deque`
//! implementations provided in the standard library.
//!
//!
//! - https://passcod.name/rast/memreader
//! - https://github.com/passcod/rast/tree/master/memstream
//! [`MemWriter`]: http://doc.rust-lang.org/std/io/struct.MemWriter.html
//! [`MemReader`]: http://doc.rust-lang.org/std/io/struct.MemReader.html

#![crate_type = "lib"]
#![license = "Public Domain"]

use std::result::{Err, Ok};
use std::collections::{Deque, DList, RingBuf};
use std::io;
use std::io::{IoResult, Stream, Reader, Writer};
use std::vec::Vec;

/// The possible modes a `MemStream` can use.
pub enum Mode { Fifo, Filo }

/// A trait for in-memory, `Deque<u8>`-backed streams.
///
/// The `_read` and `_write` provided methods are to
/// be used to implement the required methods for
/// `Reader` and `Writer`, like so:
///
///    impl Reader for SomeMemStream {
///      fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
///        self._write(buf)
///      }
///    }
///    
///    impl Writer for SomeMemStream {
///      fn write(&mut self, buf: &[u8]) -> IoResult<()> {
///        self._write(buf)
///      }
///    }
pub trait MemStream<T: Deque<u8>>: Stream {

  /// Creates a new `MemStream` in `Filo` mode.
  fn new() -> Self {
    let it: Self = MemStream::with_mode(Fifo);
    return it;
  }

  /// Creates a new `MemStream` in either `Fifo` ("first
  /// in first out", the intuitive and default behaviour)
  /// or `Filo` ("first in last out") mode.
  fn with_mode(mode: Mode) -> Self;
  
  /// Acquires an immutable reference to the underlying `Deque`.
  fn get_ref<'a>(&'a self) -> &'a T;

  /// Unwraps this `MemStream`, returning the underlying `Deque`.
  fn unwrap(self) -> T;

  /// Acquires a mutable reference to the underlying `Deque`.
  ///
  /// This is used internally, and while manipulating the
  /// underlying `Deque` will not corrupt the `MemStream`,
  /// its use in external code is discouraged.
  fn queue<'a>(&'a mut self) -> &'a mut T;

  /// Acquires an immutable reference to the `Mode` of
  /// this `MemStream`.
  fn mode<'a>(&'a self) -> &'a Mode;

  /// Implements the `read` method for the `Reader` trait.
  /// See the code sample in the trait description for
  /// implementation details.
  fn _read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
    if self.queue().is_empty() { return Err(io::standard_error(io::EndOfFile)) }

    let mut out: Vec<u8> = Vec::new();
    let mut count: uint = 0;
    for _ in buf.iter() {
      out.push(match match *self.mode() {
        Fifo => self.queue().pop_back(),
        Filo => self.queue().pop_front()
      } {
        Some(e) => { count += 1; e },
        None => 0
      });
    }

    let put: &[u8] = out.as_slice();
    assert_eq!(buf.len(), put.len());
    std::slice::bytes::copy_memory(buf, put);

    Ok(count)
  }

  /// Implements the `write` method for the `Writer` trait.
  /// See the code sample in the trait description for
  /// implementation details.
  fn _write(&mut self, buf: &[u8]) -> IoResult<()> {
    for elt in buf.iter() {
      self.queue().push_front(elt.clone());
    }
    Ok(())
  }
}

/// An implementation of `MemStream` using `DList`.
pub struct DListStream { que: DList<u8>, mode: Mode }

impl MemStream<DList<u8>> for DListStream {
  fn with_mode(mode: Mode) -> DListStream {
    let metal: DList<u8> = DList::new();
    DListStream { que: metal, mode: mode }
  }

  fn get_ref<'a>(&'a self) -> &'a DList<u8> { &'a self.que }
  fn unwrap(self) -> DList<u8> { self.que }
  fn queue<'a>(&'a mut self) -> &'a mut DList<u8> { &'a mut self.que }
  fn mode<'a>(&'a self) -> &'a Mode { &'a self.mode }
}

impl Writer for DListStream {
  fn write(&mut self, buf: &[u8]) -> IoResult<()> { self._write(buf) }
}

impl Reader for DListStream {
  fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> { self._read(buf) }
}

/// An implementation of `MemStream` using `RingBuf`.
pub struct RingBufStream { que: RingBuf<u8>, mode: Mode }

impl MemStream<RingBuf<u8>> for RingBufStream {
  fn with_mode(mode: Mode) -> RingBufStream {
    let metal: RingBuf<u8> = RingBuf::new();
    RingBufStream { que: metal, mode: mode }
  }

  fn get_ref<'a>(&'a self) -> &'a RingBuf<u8> { &'a self.que }
  fn unwrap(self) -> RingBuf<u8> { self.que }
  fn queue<'a>(&'a mut self) -> &'a mut RingBuf<u8> { &'a mut self.que }
  fn mode<'a>(&'a self) -> &'a Mode { &'a self.mode }
}

impl Writer for RingBufStream {
  fn write(&mut self, buf: &[u8]) -> IoResult<()> { self._write(buf) }
}

impl Reader for RingBufStream {
  fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> { self._read(buf) }
}
