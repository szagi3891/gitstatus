use std::fs::{self};
use std::path::Path;
use std::process::{Command, Output};



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

enum ErrStatus {
	Exec(std::io::Error),
	Output(Output),
}


fn main() {
    
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


fn test_repo(item: &PathInfo) {
	
	
	if item.is_dir == false {
		
		println!("{}", item.path);
		println!("test_repo: pomijam bo to plik");
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
	
	match exec_command(Command::new("git").arg("rev-parse").current_dir(&Path::new(&item.path))) {
		
		Ok(()) => {
			println!("{}", item.path);
			println!("ok ... dalsze przetwarzanie tego repo ...");
		}
		
		Err(ErrStatus::Exec(err)) => {
			
			println!("{}", item.path);
			println!("test_repo: ErrStatus::Exec --> {}", err);
			return;
		}
		
		Err(ErrStatus::Output(out)) => {
			
			println!("{}", item.path);
			println!("test_repo: ErrStatus::NoEmptyOutput --> {}, {:?}, {:?}", out.status, out.stdout, out.stderr);
			
			match str::from_utf8(out.stdout) {
				Ok(v) => v {
				}
				Err(e) => {
					
				}
			};
			
			return;
		}
	}
}

//fn show_u8(vec

//fn exec_command<S: Command::Command>(path: &String, command: S) {

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