use std::io;
use std::fs::{self};    //, DirEntry
use std::path::Path;



enum ErrorList {
	MainDir(io::Result<String>)
}



fn main() {
    
    println!("Hello, world! 2");
    
	let listResult = getList(&"/home/grzegorz/Pulpit".to_string());
	
	match listResult {
		
		Ok(list) => {
			println!("ok list");
		}
		Err(error) => {
			println!("err list");
		}
	}
}



fn getList(dir_str: &String) -> Result<Vec<String>, ErrorList> {
	
	
    let dir = Path::new(dir_str);	//"/home/grzegorz/Pulpit");
    
	
	match fs::metadata(dir) {
        
        Ok(dir_info) => {
			
			//Err(ErrorList::Ok)
			Ok(vec!["aa".to_string()])
			
        }
        Err(err) => {
            
			Err(ErrorList::MainDir(err))
            //panic!("błąd sprawdzania {:?}", err);
        }
    }
	
	/*
    
            if dir_info.is_dir() {
                
                match fs::read_dir(dir) {
                    
                    Ok(list) => {
                        
                        for item in list {
                            
                            match item {
                                
                                Ok(list_item) => {
                                    
									println!("czytam {:?}", list_item.path());
									
                                    match list_item.metadata() {
                                        
                                        Ok(list_item_metadata) => {
                                            
                                            if list_item_metadata.is_dir() {
                                                println!("przeczytałem katalog: {:?}", list_item.path());
                                            } else {
                                                println!("przeczytałem plik: {:?}", list_item.path());
                                            }
                                            
                                        }
                                        Err(err) => {
                                            //panic!("błąd pobieraniu metadanych {:?}", err);
											println!("błąd pobieraniu metadanych {:?}", err);
                                        }
                                    }
                                    
                                    
                                }
                                Err(err) => {
                                    panic!("błąd czytania itemu {:?}", err);
                                }
                            }
                        }
                    }
                    
                    Err(err) => {
                        panic!("błąd listowania {:?}", err);
                    }
                }
                
            } else {
                
                panic!("to nie katalog");
            }

	*/
}