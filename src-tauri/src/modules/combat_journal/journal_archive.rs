use super::journal_report::JournalReport;
use std::fs;
use std::io::Write;
use std::io::{BufReader, ErrorKind};
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub(crate) struct JournalArchive {
    path: PathBuf,
}

impl JournalArchive {
    const MAX_BYTES: usize = 32 * 1024 * 1024;

    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub(crate) fn read(&self) -> Result<Vec<JournalReport>, String> {
        let file = match fs::File::open(&self.path) {
            Ok(file) => file,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(error.to_string()),
        };

        if file.metadata().map_err(|error| error.to_string())?.len() > Self::MAX_BYTES as u64 {
            return Err("Combat archive is too large and was left unchanged".into());
        }

        let reports: Vec<JournalReport> =
            serde_json::from_reader(BufReader::new(file)).map_err(|error| {
                format!("Combat archive is damaged and was left unchanged: {error}")
            })?;

        Self::validate(&reports)?;

        Ok(reports)
    }

    pub(crate) fn save(&self, reports: Vec<JournalReport>) -> Result<(), String> {
        self.read()?;
        Self::validate(&reports)?;

        let bytes = Self::fit(reports)?;

        let parent = self
            .path
            .parent()
            .ok_or("Combat archive directory is missing")?;

        fs::create_dir_all(parent).map_err(|error| error.to_string())?;

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_nanos();
        let temporary = self.path.with_extension(format!("{nonce}.tmp"));
        let result = (|| {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)
                .map_err(|error| error.to_string())?;

            file.write_all(&bytes).map_err(|error| error.to_string())?;
            file.sync_all().map_err(|error| error.to_string())?;
            drop(file);

            fs::rename(&temporary, &self.path).map_err(|error| error.to_string())
        })();

        if result.is_err() {
            let _ = fs::remove_file(temporary);
        }

        result
    }

    /// Reports are newest first. Rather than refusing to save, the oldest fights lose their
    /// event trace first and then drop out, so new fights always reach the disk.
    fn fit(mut reports: Vec<JournalReport>) -> Result<Vec<u8>, String> {
        let size = |report: &JournalReport| {
            serde_json::to_vec(report)
                .map(|bytes| bytes.len() + 1)
                .map_err(|error| error.to_string())
        };

        let mut sizes = reports.iter().map(size).collect::<Result<Vec<_>, _>>()?;

        while sizes.iter().sum::<usize>() + 2 > Self::MAX_BYTES {
            if let Some(index) = reports.iter().rposition(JournalReport::has_trace) {
                reports[index].drop_trace();
                sizes[index] = size(&reports[index])?;
            } else if reports.len() > 1 {
                reports.pop();
                sizes.pop();
            } else {
                return Err("Combat report exceeds 32 MiB".into());
            }
        }

        serde_json::to_vec(&reports).map_err(|error| error.to_string())
    }

    fn validate(reports: &[JournalReport]) -> Result<(), String> {
        if reports.len() > 100 {
            return Err("Combat archive contains more than 100 reports".into());
        }

        for report in reports {
            report.validate()?;
        }

        Ok(())
    }
}
