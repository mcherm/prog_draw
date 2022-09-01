
use std::io;
use std::iter::zip;


pub struct FoldInfo {
    fold_paths: Vec<Vec<String>>,
}


impl FoldInfo {
    /// If any prefix of the path matches the path to a fold spot, return
    /// Some(the-position-that-folds). Otherwise, return None.
    pub fn get_fold_position(&self, path: &Vec<String>) -> Option<usize> {
        for fold_path in self.fold_paths.iter() { // consider all fold paths we know
            if fold_path.len() <= path.len() { // where the fold path isn't shorter than the path
                if zip(fold_path, path).all(|(x,y)| x == y) { // if they match up to the fold path
                    return Some(fold_path.len() - 1) // then return that position
                }
            }
        }
        None // otherwise, it didn't match
    }
}



pub fn read_fold_info(filename: &str) -> Result<FoldInfo, io::Error> {
    let mut fold_paths = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    for result in reader.records() {
        let record = result.unwrap();
        let mut fold_path: Vec<String> = Vec::new();
        for s in record.iter() {
            if s.is_empty() {
                break
            } else {
                fold_path.push(s.to_string());
            }
        }
        if fold_path.len() > 0 {
            fold_paths.push(fold_path);
        }
    }


    Ok(FoldInfo{fold_paths})
}
