use super::IconDownload;
use super::IconName;
use png::Decoder;
use std::collections::BTreeSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use uuid::Uuid;

pub(crate) struct IconCache {
    root: PathBuf,
    build: u32,
    available: RwLock<BTreeSet<String>>,
    stopped: AtomicBool,
}

impl IconCache {
    pub(crate) const MAX_BYTES: usize = 2 * 1024 * 1024;
    const MAX_PIXELS: u64 = 2048 * 2048;

    pub(crate) fn new(root: PathBuf, build: u32) -> Self {
        Self {
            root,
            build,
            available: RwLock::new(BTreeSet::new()),
            stopped: AtomicBool::new(false),
        }
    }

    pub(super) fn directory(&self) -> PathBuf {
        self.root.join(format!("v{}", self.build))
    }

    pub(crate) fn initialize(&self, plan: &[IconDownload]) -> Result<(), String> {
        Self::ensure_directory(&self.root)?;
        Self::ensure_directory(&self.directory())?;

        let _ = self.clean_old_versions();
        let mut available = BTreeSet::new();

        for file in plan.iter().flat_map(|download| &download.files) {
            if self
                .read_file(file)
                .is_some_and(|bytes| Self::valid_png(&bytes))
            {
                available.insert(file.clone());
            }
        }

        *self
            .available
            .write()
            .map_err(|_| "Icon cache unavailable")? = available;

        Ok(())
    }

    fn ensure_directory(path: &Path) -> Result<(), String> {
        if let Ok(metadata) = fs::symlink_metadata(path)
            && (metadata.file_attributes() & 0x400 != 0 || !metadata.is_dir())
        {
            return Err("Icon cache directory is not a regular directory".into());
        }

        fs::create_dir_all(path).map_err(|error| error.to_string())
    }

    fn clean_old_versions(&self) -> Result<(), String> {
        let root = self
            .root
            .canonicalize()
            .map_err(|error| error.to_string())?;

        for entry in fs::read_dir(&root).map_err(|error| error.to_string())? {
            let entry = entry.map_err(|error| error.to_string())?;
            let name = entry.file_name().to_string_lossy().into_owned();
            let Some(version) = name
                .strip_prefix('v')
                .and_then(|value| value.parse::<u32>().ok())
            else {
                continue;
            };
            let metadata = fs::symlink_metadata(entry.path()).map_err(|error| error.to_string())?;

            if version >= self.build
                || !metadata.is_dir()
                || metadata.file_attributes() & 0x400 != 0
            {
                continue;
            }

            let path = entry
                .path()
                .canonicalize()
                .map_err(|error| error.to_string())?;

            if path.parent() != Some(root.as_path()) {
                continue;
            }

            for child in fs::read_dir(&path).map_err(|error| error.to_string())? {
                let child = child.map_err(|error| error.to_string())?;
                let metadata =
                    fs::symlink_metadata(child.path()).map_err(|error| error.to_string())?;

                if metadata.is_file() && metadata.file_attributes() & 0x400 == 0 {
                    let _ = fs::remove_file(child.path());
                }
            }

            let _ = fs::remove_dir(path);
        }

        Ok(())
    }

    pub(crate) fn files(&self) -> Vec<String> {
        self.available
            .read()
            .map(|files| files.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub(crate) fn contains(&self, file: &str) -> bool {
        self.available
            .read()
            .is_ok_and(|files| files.contains(file))
    }

    fn read_file(&self, file: &str) -> Option<Vec<u8>> {
        if !IconName::valid_file(file) {
            return None;
        }

        let path = self.directory().join(file);
        let metadata = fs::symlink_metadata(&path).ok()?;

        if !metadata.is_file()
            || metadata.file_attributes() & 0x400 != 0
            || metadata.len() > Self::MAX_BYTES as u64
        {
            return None;
        }

        let mut bytes = Vec::new();
        fs::File::open(path)
            .ok()?
            .take(Self::MAX_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .ok()?;

        (bytes.len() <= Self::MAX_BYTES).then_some(bytes)
    }

    pub(crate) fn read(&self, file: &str) -> Option<Vec<u8>> {
        self.contains(file).then(|| self.read_file(file)).flatten()
    }

    pub(crate) fn valid_png(bytes: &[u8]) -> bool {
        if bytes.len() > Self::MAX_BYTES
            || !bytes.starts_with(b"\x89PNG\r\n\x1a\n")
            || !bytes.ends_with(b"\0\0\0\0IEND\xaeB`\x82")
        {
            return false;
        }

        let Ok(mut reader) = Decoder::new(Cursor::new(bytes)).read_info() else {
            return false;
        };
        let info = reader.info();

        if u64::from(info.width) * u64::from(info.height) > Self::MAX_PIXELS {
            return false;
        }

        let Some(size) = reader
            .output_buffer_size()
            .filter(|size| *size <= Self::MAX_PIXELS as usize * 8)
        else {
            return false;
        };

        reader.next_frame(&mut vec![0; size]).is_ok() && reader.finish().is_ok()
    }

    pub(crate) fn save(&self, file: &str, bytes: &[u8]) -> Result<(), String> {
        if !IconName::valid_file(file) || !Self::valid_png(bytes) {
            return Err("Invalid icon filename or PNG".into());
        }

        let target = self.directory().join(file);
        let temporary = self.directory().join(format!("{}.part", Uuid::new_v4()));
        let result = (|| -> std::io::Result<()> {
            let mut output = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&temporary)?;
            output.write_all(bytes)?;
            output.sync_all()?;
            drop(output);
            fs::rename(&temporary, &target)
        })();

        let _ = fs::remove_file(temporary);
        result.map_err(|error| error.to_string())?;
        self.available
            .write()
            .map_err(|_| "Icon cache unavailable")?
            .insert(file.to_owned());

        Ok(())
    }

    pub(crate) fn stop(&self) {
        self.stopped.store(true, Ordering::Relaxed);
    }

    pub(crate) fn stopped(&self) -> bool {
        self.stopped.load(Ordering::Relaxed)
    }
}
