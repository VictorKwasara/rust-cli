use core::num;
use std::error::Error;
use clap::{App,Arg} ;
use std::fs::File ;
use std::io::{self,BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  number_lines: bool,
  number_nonblank_lines: bool, 
}

type MyResult<T> = Result<T, Box<dyn Error>> ;

 pub fn run(config: Config) -> MyResult<()> {
  // dbg!(config);
  for filename in config.files {
    match open(&filename) {
      Err(err) => eprintln!("Failed to open {}: {}", filename, err),
      Ok(reader) => read_lines( reader, config.number_lines, config.number_nonblank_lines) ,
    }
  }

  Ok(()) 
}

fn read_lines (b_reader: Box<dyn BufRead>, n: bool, b: bool) {
   let mut  number =  1 ; 
  for line in b_reader.lines() {
    match line {
      Err(e) => eprintln!("Failed to readline {:?}", e) ,
      Ok(line) => {        
        if line == "" && b {
          println!("{}", line)
        }else if n | b {
           println!("{:>6}\t{}", number , line);
           number = number + 1
        } else {
           println!("{}", line);
        }      
      } 
    }
  }

}

fn open (filename : &str ) -> MyResult<Box<dyn BufRead>>{
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("catr")
      .version("0.1.0")
      .author("Victor Kwasara <kwasaravictort@gmail.com>")
      .about("Rust cat")
      .arg(
        Arg::with_name("files")
         .value_name("FILENAME")
         .help("Input file(s)")
         .multiple(true)
         .default_value("-")
      )
      .arg(
        Arg::with_name("number_lines")
         .short("n")
         .long("number")
         .help("Print number lines")
         .takes_value(false)
         .conflicts_with("number_nonblank_lines")
      )
      .arg(
        Arg::with_name("number_nonblank_lines")
        .short("b")
        .long("number-nonblank-lines")
        .help("Print number_nonblank_lines")
        .takes_value(false)
      )
      .get_matches();
 
    Ok(Config {
      files:matches.values_of_lossy("files").unwrap(),
      number_lines: matches.is_present("number_lines"),
      number_nonblank_lines:  matches.is_present("number_nonblank_lines"),
    })

}
