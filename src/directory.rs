use ulid::Ulid;
use std::fs;
use super::utility_error::UtilityError;
use csv::Reader;

#[derive(Debug)]
pub struct DirectoryItem {
    pub podcast_name: String,
    pub feed_url: String,
    pub podcast_id: Ulid,
}

#[derive(Debug)]
pub struct Directory {
    items: Vec<DirectoryItem>,
    location: String,
}

impl Directory {
    pub fn new(directory_location: String) -> Directory{
        Directory { items: Vec::new(), location: directory_location }
    }

    fn process_reader_data(&mut self, data: &[u8]) -> Result<(), UtilityError>{
        let mut csv_reader = Reader::from_reader(data);
        let mut errors: Vec<UtilityError> = Vec::new();
        let mut row_number: usize = 1;

        for row in csv_reader.records() {
            row_number += 1;

            if row.is_err() {
                errors.push(UtilityError::new(
                    String::from(row.err().unwrap().to_string()),
                    Some(row_number)
                ));
                continue;
            }

            let row = row.ok().unwrap();

            if row.len() != 3 {
                errors.push(UtilityError::new(
                    String::from("Row doesn't have 3 columns"),
                    Some(row_number)
                ));
                continue;
            }

            let processed_id = Ulid::from_string(&row[2]);

            if processed_id.is_err() {
                errors.push(UtilityError::new(
                    String::from(processed_id.err().unwrap().to_string()),
                    Some(row_number)
                ));
                continue;
            }

            let processed_id = processed_id.ok().unwrap();

            let new_record = DirectoryItem {
                podcast_name: String::from(&row[0]),
                feed_url: String::from(&row[1]),
                podcast_id: processed_id
            };

            self.items.push(new_record);
        }

        match errors.len() == 0 {
            true => Ok(()),
            false => Err(UtilityError::with_sub_problems(
                String::from("Issues found when reading csv lines"),
                errors,
                None
            )),
        }
    }

    pub fn read(&mut self, string_override: Option<String>) -> Result<(), UtilityError>{
        let data = match string_override {
            Some(s) => Ok(s.into_bytes()),
            None => fs::read(self.location.as_str())
        };

        match data {
            Ok(d) => self.process_reader_data(d.as_slice()),
            Err(e) => Err(UtilityError::new(String::from(e.to_string()), None))
        }
    }

    pub fn verify(&self) -> Result<(), UtilityError>{
        //return Err(UtilityError::new(String::from("Here is a problem"), None))
        Ok(())
    }

    pub fn sort (&mut self) {
        self.items.sort_by(|a, b| compare_strings_case_invariant(&a.podcast_name, &b.podcast_name));
    }

    /*pub fn write(&mut self) {
        self.sort();
        write_file
    }*/
}

fn compare_strings_case_invariant(a: &String, b: &String) -> std::cmp::Ordering {
    a.to_lowercase().as_str().cmp(b.to_lowercase().as_str())
}