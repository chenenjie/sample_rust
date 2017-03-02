#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;


mod errors {
    error_chain!{}
}


use errors::*;


fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";


        writeln!(stderr, "error: {}", e).expect(errmsg);
        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace(){
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}


fn run() -> Result<()> {
    use std::fs::File;
    use std::env;

    File::open("m secret file")
        .chain_err(|| "unable to open my secret file")?;
    
    bail!("giving up");
    
    Ok(())
}