use std;
use std::process::Output;


pub enum ErrCommand {
	Exec(std::io::Error),
	Output(Output),
	Utf8(std::string::FromUtf8Error),
}


pub fn exec_err_to_string(err: ErrCommand) -> String {
	
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



