extern crate ansi_term;

use std::path::Path;
//use std::env;

mod err_command;
mod get_list;
mod command;
mod print;

use err_command::{exec_err_to_string};
use get_list::{PathInfo, get_list};
use command::{Comm, exec_expect, exec_get, comm_to_string};
use print::{println_red, println_green};


fn main() {
	
	println!("");
	
	let exit_code = real_main();
	
	println!("\n\n");

	std::process::exit(exit_code);
}


fn real_main() -> i32 {
    
	/*
	let threads = env::var("THREADS").unwrap_or("2".to_owned())
        .parse().unwrap();
	*/
	
                            //TODO ...
    /*
    match env::current_exe() {
        Ok(exe_path) => println!("Path of this executable is: {}",
                                  exe_path.display()),
        Err(e) => println!("failed to get current exe path: {}", e),
    };
    
    parent().parent().parent().parent()
    /home/grzegorz/Pulpit/rust/gitstatus/target/debug/git-status-all
    ->
    /home/grzegorz/Pulpit/rust
    */
    
    
    
	let dir_str     = "/home/grzegorz/Pulpit/rust".to_owned();
	let root_path   = Path::new(&dir_str);
	let list_result = get_list(&root_path);
	
	
	match list_result {
		
		Ok(list) => {
			
			let mut count_ok = 0;
			let list_len     = list.len();
			
			for item in list {
				
				println!("Testuję ścieżkę: {}", item.path);
				
				match test_repo(&item) {
					Ok(()) => {
                        println_green("repo -> ok".to_owned());
						count_ok = count_ok + 1;
					}
					Err(str_err) => {
                        println_red(str_err);
					}
				}
				
				println!("");
			}
			
			
			if count_ok == list_len {
				
				println_green("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".to_owned());
				println_green("Cała lista została sprawdzona".to_owned());
				println_green("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".to_owned());
				
				0
				
			} else {
				
				let count_err = list_len - count_ok;
				let mess = format!("Cała lista została sprawdzona - błędnych {} z {}", count_err, list_len);
				
				println_red("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".to_owned());
				println_red(mess);
				println_red("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".to_owned());
				
				1
			}
			
		}
		
		Err(error) => {
			
			let mess = format!("err list {:?}", error);
								   
			println_red(mess);
			
			1
		}
	}
}



fn test_repo(item: &PathInfo) -> Result<(), String> {
	
	
	if item.is_dir == false {
		
		return Err(format!("pomijam bo to plik"));
	}
	
	
	/*
	sprawdza czy to repozytorium git
	jeśli pusty string odpowiedzi to repo jest poprawne
	*/
	
	let command1 = Comm {
		current_dir : item.path.clone(),
		command     : "git".to_owned(),
		args        : vec!["rev-parse".to_owned()],
	};
	
	
	
	try!(exec_expect(&command1, "".to_owned()));
	
	
	/*
	sprawdza czy występują nieskomitowane zmiany
	*/
	
	
	let command2 = Comm {
		current_dir : item.path.clone(),
		command     : "git".to_owned(),
		args        : vec!["status".to_owned(), "--short".to_owned()],
	};
	
	
	try!(exec_expect(&command2, "".to_owned()));
	
	
	
    
                                        //pobiera listę kolejnych branczy
	let command3 = Comm {
		current_dir : item.path.clone(),
		command     : "git".to_owned(),
		args        : vec!["branch".to_owned()],
	};
	
	match exec_get(&command3) {
		
		Ok(list_str) => {
			
			let mut is_err = false;
			
			for branch in list_str.lines() {
				
				
				let branch_clear: String = branch.to_owned().chars().filter(|&item| {
					
					if item == ' ' || item == '*' {
						false
					} else {
						true
					}
				
				}).collect();
				
				
				match test_branch(&item.path, &branch_clear) {
					
					Ok(()) => {
						
						let mess = format!("branch {} -> ok", branch_clear);
						println_green(mess);
					}
					
					Err(str) => {
						
						let mess = format!("branch {} -> {}", branch_clear, str);
						println_red(mess);
						
						is_err = true;
					}
				}
			}
			
			match is_err {
				false => {
					Ok(())
				}
				true => {
					Err("Nie wszystkie branche są wypchnięte na zdalny serwer".to_owned())
				}
			}
		}
		
		Err(err) => {
			Err(exec_err_to_string(err))
		}
	}
}


fn test_branch(path: &String, branch_clear: &String) -> Result<(), String> {
	
	let head = format!("{}...origin/{}", branch_clear, branch_clear);
	
	let command3 = Comm {
		current_dir : path.clone(),
		command     : "git".to_owned(),
		args        : vec!["rev-list".to_owned(), head, "--ignore-submodules".to_owned(), "--count".to_owned()],
	};
	
	//git rev-list HEAD...origin/master --ignore-submodules --count
	
	match exec_get(&command3) {
		
		Ok(count_str) => {
			
			let str_trim = count_str.trim().to_owned();
			
			if str_trim == "0".to_owned() {
				Ok(())
			} else {
				Err(format!("rozsynchronizowany branch: {}", str_trim))
			}
		}
		
		Err(err) => {
			
			Err(format!("błąd wykonywania\n{}\n{}", comm_to_string(&command3), exec_err_to_string(err)))
		}
	}
}


