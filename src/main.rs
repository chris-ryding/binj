use std::env;
use std::fs;
use std::process;
use std::io::Read;

#[derive(Debug)]
enum IoChannel {
  Stdio,
  File( String ),
}

#[derive(Debug)]
enum OpCode {
    Insert,
    Delete,
    Replace,
}

#[derive(Debug)]
struct Config {
    input_handle : IoChannel,
    output_handle : IoChannel,
    operations : Vec<OpCode>,
}

/*****************************************************************************/
fn main() {
    let args: Vec<String> = env::args().collect();

    let opts = Config::build( &args ).unwrap_or_else( |err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!( "opts : {opts:?}" );

    if let Err( e ) = run( opts ) {
        println!( "Application error: {e}" );
        process::exit(1);
    }

    process::exit( 0 );
}


/*****************************************************************************/
fn run( config: Config ) -> Result<(), std::io::Error> {
    match config.input_handle {
        IoChannel::Stdio => println!("Reading from stdio"),
        IoChannel::File( fname ) => println!("Reading from file {fname:?}"),
    }

    let mut in_file = fs::File::open( String::from("src/main.rs") )//config.input_handle )
                            .expect( "Unable to open input file." );

    let mut read_idx : u64 = 0;

    loop {
        let mut buf : [u8; 0x1000] = [0; 0x1000]; // need heap allocation here

        let byte_count = in_file.read( &mut buf )?;
        read_idx += byte_count as u64;

        if byte_count == 0 {
            break;
        }
    }

    match config.output_handle {
        IoChannel::Stdio => println!("Write to stdio"),
        IoChannel::File( fname ) => println!("Write to file {fname:?}"),
    }

    println!("{read_idx:?}");
    Ok(())
}

/*****************************************************************************/

impl Config {
    fn build( args: &[String] ) -> Result<Config, &'static str> {
        let mut cfg = Config {
            input_handle : IoChannel::Stdio,
            output_handle : IoChannel::Stdio,
            operations : Vec::new()
        };
        let mut idx = 0;
        for argv in args.iter() {
            println!("arg {idx}: {argv}");
            idx += 1;

            if idx == 1 {
                // Don't process the executable's name
                continue;
            }

            let mut operator = argv.chars();
            let op_code = operator.next().unwrap();
            match op_code {
                'I' => cfg.operations.push(OpCode::Insert),
                'D' => cfg.operations.push(OpCode::Delete),
                'R' => cfg.operations.push(OpCode::Replace),
                _ => eprintln!("Error: Unknown operation '{op_code}'"),
            }
        }

        Ok( cfg )
    }
}