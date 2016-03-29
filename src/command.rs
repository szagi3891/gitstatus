use std;
use std::process::Command;
use std::path::Path;
use err_command::{ErrCommand, exec_err_to_string};


#[derive(Debug)]
pub struct Comm {
	pub current_dir : String,
	pub command     : String,
	pub args        : Vec<String>,
}



pub fn exec_expect(command: &Comm, value_expect: String) -> Result<(), String> {
	
	
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

pub fn comm_to_string(command: &Comm) -> String {
	
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

pub fn exec_get(command: &Comm) -> Result<String, ErrCommand> {
	
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
