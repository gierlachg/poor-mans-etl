use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use csv::Reader;

use crate::extractor::{Extractor, ExtractorError};
use crate::record::MapRecord;

pub struct CsvExtractor {
    reader: Reader<File>,
    headers: HashMap<usize, String>,
    position: u64,
}

impl CsvExtractor {
    pub fn from(file: File) -> Result<Self, Box<dyn Error>> {
        let mut reader = Reader::from_reader(file);

        let headers = reader.headers();
        let headers = match headers.iter().next() {
            Some(headers) => headers,
            None => return Err(Box::new(ExtractorError::new("missing headers"))),
        };
        let headers = headers.iter().enumerate()
            .map(|(index, name)| (index, name.to_owned()))
            .collect();

        Ok(CsvExtractor { reader, headers, position: 0 })
    }
}

impl Extractor<MapRecord> for CsvExtractor {}

impl Iterator for CsvExtractor {
    type Item = MapRecord;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.records().next()
            .map(|record| {
                self.position = self.position + 1;
                match record {
                    Ok(record) => {
                        let values = record.iter().enumerate()
                            .map(|(value_number, value)| (self.headers.get(&value_number).unwrap().to_owned(), value.to_owned()))
                            .collect();
                        MapRecord::new(self.position, values)
                    }
                    Err(_) => MapRecord::new(self.position, HashMap::new())
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use tempfile::tempfile;

    use super::*;

    #[test]
    fn should_extract() {
        let mut file = tempfile().unwrap();
        write!(file, "Column,Another Column\n\
                      Value 1,Another Value 1\n\
                      Value 2,Another Value 2").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        let mut extracted_records: Vec<MapRecord> = CsvExtractor::from(file).unwrap().into_iter()
            .collect();

        let first_expected_record = MapRecord::new(1, vec![
            ("Column".to_string(), "Value 1".to_string()),
            ("Another Column".to_string(), "Another Value 1".to_string()),
        ].into_iter().collect());
        let second_expected_record = MapRecord::new(2, vec![
            ("Column".to_string(), "Value 2".to_string()),
            ("Another Column".to_string(), "Another Value 2".to_string()),
        ].into_iter().collect());

        assert_eq!(extracted_records.len(), 2);
        assert_eq!(extracted_records.remove(0), first_expected_record);
        assert_eq!(extracted_records.remove(0), second_expected_record);
    }
}
