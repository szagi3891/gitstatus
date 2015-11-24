use std::fs::{self};    //, DirEntry
use std::path::Path;



fn main() {
    
    println!("Hello, world!");
    
    let dir = Path::new("/home/grzegorz/Pulpit/git-status-all");
    
    match fs::metadata(dir) {
        
        Ok(dir_info) => {
            
            if dir_info.is_dir() {
                
                println!("meta - katalog istotnie");
                
                match fs::read_dir(dir) {
                    
                    Ok(list) => {
                        
                        for item in list {
                            
                            println!("pierwszy plik");
                            
                            match item {
                                
                                Ok(list_item) => {
                                    
                                    match list_item.metadata() {
                                        
                                        Ok(list_item_metadata) => {
                                            
                                            if list_item_metadata.is_dir() {
                                                println!("przeczytałem katalog: {:?}", list_item.path());
                                            } else {
                                                println!("przeczytałem plik: {:?}", list_item.path());
                                            }
                                            
                                        }
                                        Err(err) => {
                                            panic!("błąd pobieraniu metadanych {:?}", err);
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
            
            
        }
        Err(err) => {
            
            panic!("błąd sprawdzania {:?}", err);
        }
    }
    
}
