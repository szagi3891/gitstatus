use std;
use std::path::Path;
use std::fs::{self};


#[derive(Debug)]
pub enum ErrGetList {
	MainDir(std::io::Error),
	MainDirNoDir,
	MainListError(std::io::Error),
	ItemGet(std::io::Error),
	ItemMetadata(String, std::io::Error),
	PathUtf8(std::path::PathBuf),			//błąd konwersji ścieżki na utf8
}


#[derive(Debug)]
pub struct PathInfo {
	pub path : String,
	pub is_dir : bool,
}


pub fn get_list(dir_str: &Path) -> Result<Vec<PathInfo>, ErrGetList> {	//Result<Vec<String>, ErrorList> {
	
	
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