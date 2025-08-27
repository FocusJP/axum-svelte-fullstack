use crate::data_file::DataFile;
use anyhow::Context;
use std::io::{Cursor, Read};
use zip::ZipArchive;

const DATA_FILE_PREFIX: &str = "yob";
const DATA_FILE_MAX_SIZE: usize = 10_000_000;
const ZIP_FILE_NAME_SIZE: usize = "yobYYYY.txt".len();

pub struct ZipIterator {
    current_index: usize,
    zip_archive: ZipArchive<Cursor<Vec<u8>>>,
}

impl ZipIterator {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let body = std::fs::read(path).context("Filed to read zip file: {:#}")?;

        tracing::debug!("Read zip from {path} with {} bytes", body.len());

        let zip_reader = Cursor::new(body);
        let zip_archive = ZipArchive::new(zip_reader)?;
        let current_index = 0;

        Ok(Self {
            current_index,
            zip_archive,
        })
    }
}

impl Iterator for ZipIterator {
    type Item = DataFile;

    fn next(&mut self) -> Option<Self::Item> {
        let archive_len = self.zip_archive.len();

        while self.current_index < archive_len {
            let mut zip_file = self.zip_archive.by_index(self.current_index).unwrap();
            self.current_index += 1;

            let zip_file_name = zip_file.name().to_string();
            let zip_file_size = zip_file.size() as usize;

            if zip_file_size > DATA_FILE_MAX_SIZE {
                tracing::error!(
                    "Not extracting {} - large decompressed size {} bytes!",
                    zip_file_name,
                    zip_file_size
                );
                continue;
            }

            if !zip_file_name.starts_with(DATA_FILE_PREFIX) {
                tracing::debug!("Not extracting {} - not a data file", zip_file_name);
                continue;
            }

            if zip_file_name.len() != ZIP_FILE_NAME_SIZE {
                tracing::error!("Unexpected data file name size: {zip_file_name}");
                continue;
            }

            let data_year = zip_file_name[3..7].parse::<i32>();
            let Ok(data_year) = data_year else {
                tracing::error!("Failed to parse year from data file name: {zip_file_name}");
                continue;
            };

            tracing::debug!("Extracting {}", zip_file_name);

            let mut file_text = String::with_capacity(zip_file_size + 1);
            zip_file.read_to_string(&mut file_text).unwrap();

            let data_file = DataFile {
                name: zip_file_name,
                text: file_text,
                year: data_year,
            };

            return Some(data_file);
        }

        None
    }
}
