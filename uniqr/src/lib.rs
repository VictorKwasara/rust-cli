use clap::{App, Arg};
use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Write}} ;

type MyResult<T> = Result<T, Box<dyn Error>> ;

#[derive(Debug)]
pub struct Config {
  in_file: String, 
  out_file: Option<String>,
  count: bool,
}

pub fn run (config: Config) -> MyResult<()> {
  // dbg!(cfg) ; 

  let mut file = open(&config.in_file)
   .map_err(|e| format!("{}: {}", config.in_file, e))?;
  let mut line = String::new() ;
  // let mut prev_line = String::new() ;
  let mut str_vec: Vec<(String, usize)>= Vec::new() ;
  let mut count = 0 ;
  loop {
   let bytes = file.read_line(&mut line)?;    
    if bytes == 0 {
     break;
    }  
    let len = str_vec.len() ;
    if count > 0  && config.count  {
     if  str_vec[len-1].0.trim_end() == line.trim_end() {
      str_vec[len-1].1 = str_vec[len-1].1 + 1;
    }else {
     str_vec.push((line.clone(), 1));
    }     
   }else if count > 0   {
     if str_vec[len-1].0.trim_end() != line.trim_end() {
     str_vec.push((line.clone(), 1));
     }     
   }else {
    str_vec.push((line.clone(), 1));
   }   
   line.clear();
   count+=1 ;
  } 

  let mut out: Box<dyn Write> = if config.out_file.is_some() {
   Box::new(File::create(config.out_file.unwrap()).unwrap())
  }else {
    Box::new(io::stdout())
  };

  for st in str_vec.iter() {
    if config.count {
      write!(out,"{:>4} {}", st.1 , st.0)?    
    }else {
      write!(out,"{}", st.0)?
    }
  }
  Ok(())
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("uniqr")
   .version("0.1.0")
   .author("Victor Kwasara <kwasaravictort@gmail.com>")
   .about("Rust uniq")
   .arg(
     Arg::with_name("in_file")
     .value_name("IN_FILE")
     .multiple(false)
     .default_value("-")
     .help("Input file")
   )
   .arg(
     Arg::with_name("out_file")
     .value_name("OUT_FILE")
     .help("Output file optional")
     .multiple(false)
   )
   .arg(
     Arg::with_name("count")
     .short("c")
     .long("count")
     .help("The number of time value occurs")
     .takes_value(false)
   )
   .get_matches() ;
  Ok(Config {
      in_file: matches.value_of("in_file").unwrap().to_string(),
      out_file: matches.value_of("out_file").map(String::from),
      count: matches.is_present("count")
  })
}

fn open (filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
    "-" => Ok(Box::new(BufReader::new(io::stdin()))), 
    _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
  }
}


