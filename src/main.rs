use std::fs::{self};
use std::path::Path;
use std::process::Command;



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



fn main() {
    
	
	let list_result = get_list(&"/home/grzegorz/Pulpit/rust".to_string());
	
	match list_result {
		
		Ok(list) => {
			
			for item in list {
				test_repo(&item);
			}
		}
		
		Err(error) => {
			
			println!("err list {:?}", error);
		}
	}
}


fn test_repo(item: &PathInfo) {
	
	
	if item.is_dir == false {
		
		println!("plik - pomijam: {}", item.path);
		return;
	}
	
	
	let output = Command::new("git").arg("rev-parse").output();
	
	
	match output {
		Ok(out) => {
			
			if out.status.success() && out.stdout.len() == 0 && out.stderr.len() == 0 {
				
				println!("sukces: {}", item.path);
				
			} else {
				panic!("problem ze statusem repo: {}", item.path);
			}
		}
		Err(err) => {
			println!("err : {:?}", err);
		}
	}
	
	
	
	/*
	let output = Command::new("sh")
						 .arg("-c")
						 .arg("echo hello")
						 .output()
	*/
	
	/*
	let status = Command::new("ls").status().unwrap_or_else(|e| {
		panic!("failed to execute process: {}", e)
	});

	println!("process exited with: {}", status);
	*/
}


fn get_list(dir_str: &String) -> Result<Vec<PathInfo>, ErrGetList> {	//Result<Vec<String>, ErrorList> {
	
	
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
								
								match list_item.metadata() {
									
									Ok(list_item_metadata) => {
										
										if list_item_metadata.is_dir() {
											out.push(PathInfo{path: list_item.path().to_str().unwrap().to_string(), is_dir : true});
										} else {
											out.push(PathInfo{path: list_item.path().to_str().unwrap().to_string(), is_dir : false});
										}
									}
									Err(err) => {	
										return Err(ErrGetList::ItemMetadata(list_item.path().to_str().unwrap().to_string(), err));
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