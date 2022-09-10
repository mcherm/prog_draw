
use std::io;
use std::iter::zip;


pub struct FoldInfo {
    fold_paths: Vec<Vec<String>>,
}


impl FoldInfo {
    /// Returns true if the path, up to "len" (and ignoring anything past that) should be
    /// folded.
    pub fn is_fold_path(&self, path: &Vec<String>, len: usize) -> bool {
        for fold_path in self.fold_paths.iter() { // consider all fold paths we know
            if fold_path.len() == len { // but only ones that are exactly this long
                if zip(fold_path, path).all(|(x,y)| x == y) { // if they match (up to the len)
                    return true // then return that position
                }
            }
        }
        false
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
