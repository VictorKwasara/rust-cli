use clap::{App, Arg};
use std::error::Error;
use std::io::{self, BufReader, BufRead, Read} ;
use std::fs::File ;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  lines: bool,
  words: bool,
  bytes: bool,
  chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
  num_lines: usize,
  num_words: usize,
  num_bytes: usize, 
  num_chars: usize, 
}

pub fn run (config: Config ) -> MyResult<()>{
  let mut total_lines = 0;
  let mut total_words = 0;
  let mut total_bytes = 0;
  let mut total_chars = 0; 
  let len = config.files.len() ;
  for ( index, filename) in config.files.into_iter().enumerate(){
      match open(&filename) {
          Err(err) => eprintln!("{}: {}", filename, err) ,
          Ok(c) => {
            let file_info = count(c)?;
            let mut  s = String::new() ;            
            s = s + format_field(file_info.num_lines, config.lines).as_str() ;
            s = s + format_field(file_info.num_words, config.words).as_str() ;
            s = s + format_field(file_info.num_bytes, config.bytes).as_str() ;
            s = s + format_field(file_info.num_chars, config.chars).as_str() ;
            let f = if filename == "_" { "".to_string() }else {format!(" {filename}")} ;
            println!("{s}{f}");
            if len > 1 {
             total_lines +=  file_info.num_lines; 
             total_words +=  file_info.num_words;
             total_bytes +=  file_info.num_bytes;
             total_chars +=  file_info.num_chars;  
            }
            if len > 1 && len == index + 1 {
             let mut  s = String::new() ;           
             s = s + format_field(total_lines, config.lines).as_str() ;
             s = s + format_field(total_words, config.words).as_str() ;
             s = s + format_field(total_bytes, config.bytes).as_str() ;
             s = s + format_field(total_chars, config.chars).as_str() ;
             println!("{s} total");
            }  
          }
      }
  } 
  Ok(())
} 

fn format_field(value: usize, show: bool) -> String {
  if show {
    format!("{:>8}", value)
  }else {
    "".to_string()
  }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
   "_" => Ok(Box::new(BufReader::new(io::stdin()))),
    _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut st = String::new();
    loop {      
      let bytes = file.read_line(&mut st)?;      
      if bytes == 0 {
        break;
      }
      num_bytes += bytes ; 
      num_lines += 1;  
      num_chars += st.chars().count();
      num_words += st.split_whitespace().count() ;        
      st.clear();
    } 
    Ok(FileInfo {
      num_bytes,
      num_lines,
      num_words,
      num_chars,
    })
}

pub fn get_args() -> MyResult<Config>{
 let matches = App::new("wcr")
  .version("0.1.0")
  .author("Victor Kwasara <kwasaravictort@gmail.com>")
  .about("Rust wc")
  .arg(
    Arg::with_name("files")
     .value_name("FILES")
     .default_value("_")
     .multiple(true)
     .help("Input file(s)")   
  )
  .arg(
    Arg::with_name("lines")
    .value_name("LINES")
    .short("l")
    .long("lines")
    .takes_value(false)
    .help("Show line count")
  )
  .arg(
    Arg::with_name("words")
      .short("w")
      .long("words")
      .takes_value(false)
      .help("Show word count")
  )
  .arg(
    Arg::with_name("bytes")
      .short("c")
      .long("bytes")
      .takes_value(false)
      .help("Show byte count")
     
  )
  .arg(
    Arg::with_name("chars")
     .short("m")
     .long("chars")
     .takes_value(false)
     .help("Show character count")  
     .conflicts_with("bytes")       
  )
  .get_matches();

    let mut lines =  matches.is_present("lines") ;
    let mut words =   matches.is_present("words") ;
    let mut bytes =  matches.is_present("bytes") ;
    let chars =  matches.is_present("chars") ;

  if [lines, words, bytes, chars].iter().all(|v| v== &false) {
      lines = true ;
      words = true ;
      bytes = true ;
  }

  Ok(
     Config {
      files: matches.values_of_lossy("files").unwrap(),
      lines,
      words,
      bytes,
      chars,
    } 
  )
}

#[cfg(test)]
mod tests {
  use super::{count, FileInfo} ;
  use std::io::Cursor ;

  #[test]
  fn test_count() {
    let text  = "I don't want the world. I just want your half.\r\n";
    let info = count(Cursor::new(text));
    assert!(info.is_ok());
    let expected = FileInfo {
      num_lines: 1,
      num_words: 10,
      num_chars: 48,
      num_bytes: 48, 
    };
    assert_eq!(info.unwrap(), expected);
  }
}