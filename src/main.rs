use std::fs::{self};
use std::path::Path;
use std::process::{Command, Output};

extern crate ansi_term;

use ansi_term::Colour::{Red, Green};		//Blue, Yellow, 


#[derive(Debug)]
enum ErrGetList {
	MainDir(std::io::Error),
	MainDirNoDir,
	MainListError(std::io::Error),
	ItemGet(std::io::Error),
	ItemMetadata(String, std::io::Error),
	PathUtf8(std::path::PathBuf),			//błąd konwersji ścieżki na utf8
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

#[derive(Debug)]
struct Comm {
	current_dir : String,
	command     : String,
	args        : Vec<String>,
}


fn main() {
	
	println!("");
	
	let exit_code = real_main();
	
	println!("\n\n");

	std::process::exit(exit_code);
}

//TODO - parsowanie argumentu



//format_args!("hello {}", "world")
//let out = format!("hello {}", "world");

/*
let four_lines = "foo\r\nbar\n\nbaz\n";
let v: Vec<&str> = four_lines.lines().collect();
assert_eq!(v, ["foo", "bar", "", "baz"]);
"lion::tiger::leopard".split("::").collect(); -> iterator

std::str::from_chars(char_vector.as_slice()); // creates a ~str
char_vector.iter().map(|c| *c).collect::<std::strbuf::StrBuf>();
Docs: std::str::from_chars
(The latter can be used to collect to a ~str too, just by changing the StrBuf type hint.)

let words = ["alpha", "beta", "gamma"];
let merged: String = words.iter()
						  .flat_map(|s| s.chars())
						  .collect();
assert_eq!(merged, "alphabetagamma");
*/


fn real_main() -> i32 {
    
	/*
	let threads = env::var("THREADS").unwrap_or("2".to_string())
        .parse().unwrap();
	*/
	
	let dir_str     = "/home/grzegorz/Pulpit/rust".to_string();
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
						println!("{}", Green.paint("repo -> ok"));
						count_ok = count_ok + 1;
					}
					Err(str_err) => {
						println!("{}", Red.paint(str_err));
					}
				}
				
				println!("");
			}
			
			
			if count_ok == list_len {
				
				println_green("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".to_string());
				println_green("Cała lista została sprawdzona".to_string());
				println_green("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".to_string());
				
				return 0;
				
			} else {
				
				let count_err = list_len - count_ok;
				let mess = format!("Cała lista została sprawdzona - błędnych {} z {}", count_err, list_len);
				
				println_red("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".to_string());
				println_red(mess);
				println_red("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".to_string());
				
				return 1;
			}
			
		}
		
		Err(error) => {
			
			let mess = format!("err list {:?}", error);
								   
			println_red(mess);
			
			return 1;
		}
	}
	
	
}


fn println_red(str: String) {
	println!("{}", Red.paint(str));
}

fn println_green(str: String) {
	println!("{}", Green.paint(str));
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
		command     : "git".to_string(),
		args        : vec!["rev-parse".to_string()],
	};
	
	
	
	try!(exec_expect(&command1, "".to_string()));
	
	
	/*
	sprawdza czy występują nieskomitowane zmiany
	*/
	
	
	let command2 = Comm {
		current_dir : item.path.clone(),
		command     : "git".to_string(),
		args        : vec!["status".to_string(), "--short".to_string()],
	};
	
	
	try!(exec_expect(&command2, "".to_string()));
	
	
	
	let command3 = Comm {
		current_dir : item.path.clone(),
		command     : "git".to_string(),
		args        : vec!["branch".to_string()],
	};
	
	match exec_get(&command3) {
		
		Ok(list_str) => {
			
			let mut is_err = false;
			
			for branch in list_str.lines() {
				
				
				let branch_clear: String = branch.to_string().chars().filter(|&item| {
					
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
					Err("Nie wszystkie branche są wypchnięte na zdalny serwer".to_string())
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
		command     : "git".to_string(),
		args        : vec!["rev-list".to_string(), head, "--ignore-submodules".to_string(), "--count".to_string()],
	};
	
	//git rev-list HEAD...origin/master --ignore-submodules --count
	
	match exec_get(&command3) {
		
		Ok(count_str) => {
			
			let str_trim = count_str.trim().to_string();
			
			if str_trim == "0".to_string() {
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


fn exec_expect(command: &Comm, value_expect: String) -> Result<(), String> {
	
	
	match exec_get(command) {
		
		Ok(mess) => {
			if mess == value_expect {
				Ok(())
			} else {
				
				let comm = comm_to_string(command);
				
				Err(format!("spodziewano się innej wartości:\n{}\n -->\n{}", comm, mess))
			}
		}
		
		Err(err_status) => {
			
			let comm        = comm_to_string(command);
			let err_message = exec_err_to_string(err_status);
			
			Err(format!("błąd wykonywania polecenia:\n{}\n -->\n{}", comm, err_message))
		}
	}
}

fn exec_err_to_string(err: ErrCommand) -> String {
	
	match err {
		
		ErrCommand::Exec(err) => {
			
			format!("ErrStatus::Exec --> {}", err)
		}
		
		ErrCommand::Output(out) => {
			
			//zrobić formatowanie kodu odpowiedzi
			
			let stdout = match String::from_utf8(out.stdout) {
				Ok(str) => str,
				Err(err) => format!("incorrect utf8: <{}>", err),
			};
			
			let stderr = match String::from_utf8(out.stderr) {
				Ok(str) => str,
				Err(err) => format!("incorrect utf8: <{}>", err),
			};
			
			let stdout_len = stdout.len();
			let stderr_len = stderr.len();
			
			format!("ErrStatus::NoEmptyOutput\n{}\nstdout len({}) -->\n{}\nstderr len({}) -->\n{}", out.status, stdout_len, stdout, stderr_len, stderr)
		}
		
		ErrCommand::Utf8(err_utf) => {
			
			format!("ErrStatus::Utf8 --> {}", err_utf)
		}
	}
}


fn comm_to_string(command: &Comm) -> String {
	
	let mut out:Vec<String> = vec![];
	
	out.push(command.command.clone());
	
	for arg in command.args.clone() {
		out.push(arg);
	}
	
	out.push("in".to_string());
	
	out.push(command.current_dir.clone());
	
	out.join(" ")
}


fn get_output(command: &Comm) -> Result<std::process::Output, std::io::Error> {
	
	let mut comm = Command::new(command.command.clone());
	
	comm.current_dir(&Path::new(&command.current_dir));
	
	for arg in command.args.clone() {
		comm.arg(arg);
	}
	
	comm.output()
}

fn exec_get(command: &Comm) -> Result<String, ErrCommand> {
	
	let output = get_output(command);
	
	match output {
		
		Ok(out) => {
			
			if out.status.success() && out.status.code() == Some(0) && out.stderr.len() == 0 {
				
				match String::from_utf8(out.stdout) {
					Ok(str) => Ok(str),
					Err(err) => Err(ErrCommand::Utf8(err))
				}
			
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
								
								let path = list_item.path();
								
								let path_item = match path.to_str() {
									Some(value) => {
										value.to_string()
									}
									None => {
										return Err(ErrGetList::PathUtf8(path.clone()))
									}
								};
								
								
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