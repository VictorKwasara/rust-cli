use clap::{App,Arg};
use std::error::Error; 
use std::fmt::format;
use std::fs::File ;
use std::io::{self,BufRead, BufReader,Read} ;

type MyResult<T> = Result<T,Box<dyn Error>> ;

#[derive(Debug)]
pub struct Config {
  files: Vec<String> ,
  lines: usize, 
  bytes: Option<usize>,
}

pub fn run(cfg: Config ) -> MyResult<()> {
  let len  = cfg.files.len() ;
  for (n, filename) in cfg.files.into_iter().enumerate() {
    match open(&filename) {
      Err(err) => eprintln!("Failed to open {}: {}", filename, err),
      Ok(mut reader) if cfg.bytes.is_some() => print_bytes (reader, cfg.bytes.unwrap(), len, &filename, n)?,
      Ok(mut reader) => print_lines(reader, cfg.lines, len, &filename, n)? ,
    }
  }
  Ok(())
 }

fn print_bytes (b_reader: Box<dyn BufRead>, b:usize, f: usize, name: &str, index: usize)-> MyResult<()> { 
 if f > 1 {
  println!("==> {name} <==")
 }
 let mut handle = b_reader.take(b as u64);
 let mut buffer = vec![0; b] ;
 let bytes_read = handle.read(&mut buffer)? ;
 print!(
  "{}",
  String::from_utf8_lossy(&buffer[..bytes_read])
 );

  if f > 1 && index+1 < f {
  println!("")
 }
Ok(())
}

fn print_lines (mut b_reader: Box<dyn BufRead>, n:usize, f: usize, name: &str, index: usize) -> MyResult<()> {
 if f > 1 {
  println!("==> {name} <==")
 }
 let mut line = String::new();
 for _  in  0..n {   
   let bytes = b_reader.read_line(&mut line)?;  
   if bytes == 0 {
    break ;
   }
   print!("{}", line);
   line.clear();
 }
 if f > 1 && index+1 < f{
  println!("")
 }

 Ok(())
}

fn open (filename: &str ) -> MyResult<Box<dyn BufRead>> {
   match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
   }
}
pub fn get_args() -> MyResult<Config> {
  let matches = App::new("headr")
   .version("0.1.0")
   .author("Victor Kwasara <kwasaravictort@gmail.com>")
   .about("Rust head")
   .arg(
    Arg::with_name("files")
     .value_name("FILES")
     .default_value("-")
     .multiple(true)
     .help("Input file name(s)")
   )
   .arg(
    Arg::with_name("lines")
     .short("n")
     .value_name("LINES")
     .long("lines")
     .takes_value(true)
     .help("The number of lines to view ")
     .default_value( "10")
   )
   .arg(
    Arg::with_name("bytes")
     .help("The number of bytes to view")
     .value_name("BYTES")
     .short("c")
     .long("bytes")
     .conflicts_with("lines")
     .takes_value(true)

   )
   .get_matches() ;

  let n =  matches.value_of("lines")
    .map(parse_positive_int)
    .transpose()
    .map_err(|e| format!("illegal line count -- {}", e)).unwrap().unwrap();

  let b =  matches.value_of("bytes").map(parse_positive_int).transpose().map_err(|e| format!("illegal byte count -- {}", e))?;
  
  Ok(Config {
    files: matches.values_of_lossy("files").unwrap() ,
    lines: n,
    bytes: b,
  })
}

fn parse_positive_int(val: &str) -> MyResult<usize> { 
  match  val.parse() {   
    Ok (v) if v > 0 => Ok(v),
    _ => Err(From::from(val)) ,
  } 
}

#[test]
fn test_parse_positive_int() {
  //3 is an OK integer 
  let res = parse_positive_int("3");
  assert!(res.is_ok());
  assert_eq!(res.unwrap(), 3);

  //Any string is an error
  let res = parse_positive_int("foo");
  assert!(res.is_err());
  assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

  // A zero is an error
  let res = parse_positive_int("0");
  assert!(res.is_err());
  assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}