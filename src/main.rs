use std::fs::{self};
use std::path::Path;
use std::process::{Command, Output};
use std::str;

extern crate ansi_term;

use ansi_term::Colour::{Blue, Yellow, Red};


#[derive(Debug)]
enum ErrGetList {
	MainDir(std::io::Error),
	MainDirNoDir,
	MainListError(std::io::Error),
	ItemGet(std::io::Error),
	ItemMetadata(String, std::io::Error),
}

#[derive(Debug)]
struct PathInfo {
	path : String,
	is_dir : bool,
}

enum ErrCommand {
	Exec(std::io::Error),
	Output(Output),
	Utf8(std::string::FromUtf8Error),
}



fn main() {
    
	println!("This is in red: {}", Red.paint("a red string"));
	
	println!("\n");
	
	
	let dir_str     = "/home/grzegorz/Pulpit/rust".to_string();
	let root_path   = Path::new(&dir_str);
	let list_result = get_list(&root_path);
	
	
	match list_result {
		
		Ok(list) => {
			
			for item in list {
				test_repo(&item);
				println!("\n");
			}
		}
		
		Err(error) => {
			
			println!("err list {:?}", error);
		}
	}
}

/*
fn show_color_red() -> String {
}
*/

/*
let mut a : String = "A".into();
let b : String = "B".into();

a.push_str(&b);
*/


fn test_repo(item: &PathInfo) {
	
	
	if item.is_dir == false {
		
		let mess1 = Yellow.paint("test_repo:");
		let mess2 = Yellow.paint(item.path.clone());
		let mess3 = Yellow.paint(" - pomijam bo to plik");
		
		println!("{} {} {}", mess1, mess2, mess3);
		
		return;
	}
	
	
	/*
	let command = Command::new("git")
		.arg("rev-parse")
		.current_dir(&Path::new(&item.path));
	
	match exec_command(&mut command) {	//.current_dir(&root_path)) {
	*/
	
	/*
	exec, komenda, spodziewany output
			-> bool			- udane lub nie
	
	output <- exec, komenda
		spodziewamy się tylko outputu, w pozostałych przypadkach błąd
	
	show output
	*/
	
	match exec_get(Command::new("git").arg("rev-parse").current_dir(&Path::new(&item.path))) {
		
		Ok(mess) => {
			println!("{}", item.path);
			println!("ok ... dalsze przetwarzanie tego repo ... {}", mess);
		}
		
		Err(ErrCommand::Exec(err)) => {
			
			println!("{}", item.path);
			println!("test_repo: ErrStatus::Exec --> {}", err);
			return;
		}
		
		Err(ErrCommand::Output(out)) => {
			
			println!("{}", item.path);
			println!("test_repo: ErrStatus::NoEmptyOutput --> {}, {:?}, {:?}", out.status, out.stdout, out.stderr);
			
			/*
			match str::from_utf8(out.stdout) {
				Ok(v) => v {
				}
				Err(e) => {
					
				}
			};
			*/
			
			return;
		}
		
		Err(ErrCommand::Utf8(errUtf)) => {
			
			println!("{}", item.path);
			println!("test_repo: ErrStatus::Utf8 --> {}", errUtf);
		}
	}
}

//fn show_u8(vec

//fn exec_command<S: Command::Command>(path: &String, command: S) {

/*
fn exec_command(command: &mut Command) -> Result<(), ErrCommand> {
	
	let output = command.output();
	
	match output {
		
		Ok(out) => {
			
			if out.status.success() && out.stdout.len() == 0 && out.stderr.len() == 0 {
				
				Ok(())
				
			} else {	
				Err(ErrCommand::Output(out))
			}
		}
		Err(err) => {
			Err(ErrCommand::Exec(err))
		}
	}
}
*/


fn exec_get(command: &mut Command) -> Result<String, ErrCommand> {
	
	let output = command.output();
	
	match output {
		
		Ok(out) => {
			
			if out.status.success() && out.status.code() == Some(0) && out.stderr.len() == 0 {
				
				//&& out.stdout.len() == 0 && out.stderr.len() == 0
				
				//Ok(out.stdout.to_string())
				
				//format_args!("hello {}", "world")
				
				match String::from_utf8(out.stdout) {
					Ok(str) => Ok(str),
					Err(err) => Err(ErrCommand::Utf8(err))
				}
				
				//-> Result<String, FromUtf8Error>
				
				
				/*
				match str::from_utf8(out.stdout) {
					Ok(str) => Ok(str),
					Err(err) => panic!("Invalid UTF-8 sequence: {}", err),
				}
				*/
				//format_args!("hello {}", "world")
				
				//println!("result: {}", s);
				
				//Ok("ok ...".to_string())
				
			} else {	
				Err(ErrCommand::Output(out))
			}
		}
		Err(err) => {
			Err(ErrCommand::Exec(err))
		}
	}
}


fn get_list(dir_str: &Path) -> Result<Vec<PathInfo>, ErrGetList> {	//Result<Vec<String>, ErrorList> {
	
	
    let dir = Path::new(dir_str);	//"/home/grzegorz/Pulpit");
    
	
	match fs::metadata(dir) {
        
        Ok(dir_info) => {
			
			if dir_info.is_dir() == false {
				return Err(ErrGetList::MainDirNoDir);
			}
                
			match fs::read_dir(dir) {
				
				Ok(list) => {
					
					let mut out: Vec<PathInfo>= vec![];
					
					for item in list {
						
						match item {
							
							Ok(list_item) => {
								
														//TODO - trzeba się pozbyć unwrap
								
								let path_item = list_item.path().to_str().unwrap().to_string();
								
								
								match list_item.metadata() {
									
									Ok(list_item_metadata) => {
										
										if list_item_metadata.is_dir() {
											out.push(PathInfo{path: path_item, is_dir : true});
										} else {
											out.push(PathInfo{path: path_item, is_dir : false});
										}
									}
									Err(err) => {	
										return Err(ErrGetList::ItemMetadata(path_item, err));
									}
								}
							}
							Err(err) => {
								return Err(ErrGetList::ItemGet(err))
							}
						};
					};
					
					Ok(out)
									
				}
				Err(err) => {
					Err(ErrGetList::MainListError(err))
				}
			}
		
        }
        Err(err) => {
			Err(ErrGetList::MainDir(err))
        }
    }
}