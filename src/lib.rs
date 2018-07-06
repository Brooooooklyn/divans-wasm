#![feature(proc_macro, wasm_import_module, wasm_custom_section)]
extern crate divans;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use std::io;
use std::io::Write;

#[wasm_bindgen]
pub fn compress(quality: u16, mut input: &[u8], output: &mut [u8]) {
  let mut opts = divans::DivansCompressorOptions::default();
  opts.quality = Some(quality);
  {
    let mut writer = divans::DivansBrotliHybridCompressorWriter::new(
      output,
      opts,
      4096, // internal buffer size
    );
    io::copy(&mut input, &mut writer).unwrap();
    writer.flush().unwrap();
  }
}
