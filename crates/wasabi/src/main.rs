use std::{fs, io};

use main_error::MainError;
use structopt::StructOpt;
use wasabi_wasm::Module;

use wasabi::instrument::add_hooks;
use wasabi::options::{HookSet, Options};

// TODO use failure crate and failure::Error type for error handling or use custom error trait
// TODO remove most, if not all unwrap() and panic!()
// Error kinds:
// - OptionsError: invalid option...
// - ParseError: error parsing wasm file at offset 0x0000: unknown upcode 0xf7f7 (test with SIMD file)
// - TypeError: cannot type check...

fn main() -> Result<(), MainError> {
    let opt = Options::from_args();

    let mut enabled_hooks = if opt.hooks.is_empty() {
        // If --hooks is not given, everything shall be instrumented.
        HookSet::all()
    } else {
        let mut enabled_hooks = HookSet::new();
        for hook in opt.hooks {
            enabled_hooks.insert(hook);
        }
        enabled_hooks
    };
    for hook in opt.no_hooks {
        enabled_hooks.remove(hook);
    }

    let input_filename = opt.input_file.file_name().ok_or_else(|| io_err("invalid input file, has no filename"))?;
    let output_file_wasm = opt.output_dir.join(input_filename);
    let output_file_wasabi_js = output_file_wasm.with_extension("wasabi.js");

    // instrument Wasm and generate JavaScript
    let (mut module, _offsets, _warnings) = Module::from_file(opt.input_file)?;
    let (js, hook_count) = add_hooks(&mut module, enabled_hooks, opt.node_js).unwrap();
    println!("inserted {} low-level hooks", hook_count);

    // write output files
    fs::create_dir_all(&opt.output_dir)?;
    module.to_file(output_file_wasm)?;
    fs::write(output_file_wasabi_js, js)?;
    if opt.node_js {
        let output_file_long_js = opt.output_dir.join("long.js");
        fs::write(output_file_long_js, include_str!("../js/long.js/long.js"))?;
    }

    Ok(())
}

// TODO remove after proper error handling
fn io_err(str: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, str.to_string())
}
