use super::IconCache;
use super::IconDownload;
use super::IconResponse;
use super::IconTransport;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

pub(crate) struct IconLoader;

impl IconLoader {
    const WORKERS: usize = 4;

    pub(crate) fn run(
        cache: &IconCache,
        plan: &[IconDownload],
        transport: &dyn IconTransport,
        changed: &(dyn Fn(Vec<String>) + Sync),
    ) -> Result<(), String> {
        cache.initialize(plan)?;
        changed(cache.files());
        let next = AtomicUsize::new(0);
        let offline = AtomicBool::new(false);
        let errors = Mutex::new(Vec::new());

        thread::scope(|scope| {
            for _ in 0..Self::WORKERS {
                scope.spawn(|| {
                    while !cache.stopped() && !offline.load(Ordering::Relaxed) {
                        let Some(download) = plan.get(next.fetch_add(1, Ordering::Relaxed)) else {
                            break;
                        };

                        if download.files.iter().all(|file| cache.contains(file)) {
                            continue;
                        }

                        let cached = download.files.iter().find_map(|file| cache.read(file));
                        let response = cached
                            .map(IconResponse::Image)
                            .unwrap_or_else(|| transport.fetch(&download.name));
                        let result = match response {
                            IconResponse::Image(bytes) => {
                                Self::save(cache, download, &bytes, changed)
                            }
                            IconResponse::Missing => Ok(()),
                            IconResponse::Retry(error) => Err(error),
                            IconResponse::Offline(error) => {
                                offline.store(true, Ordering::Relaxed);
                                Err(error)
                            }
                        };

                        if let Err(error) = result
                            && let Ok(mut errors) = errors.lock()
                        {
                            errors.push(error);
                        }
                    }
                });
            }
        });

        let errors = errors.into_inner().map_err(|_| "Icon loader unavailable")?;

        if let Some(first) = errors.first() {
            return Err(format!("{} failure(s); {first}", errors.len()));
        }

        Ok(())
    }

    fn save(
        cache: &IconCache,
        download: &IconDownload,
        bytes: &[u8],
        changed: &(dyn Fn(Vec<String>) + Sync),
    ) -> Result<(), String> {
        let mut added = Vec::new();

        for file in &download.files {
            if !cache.contains(file) && !cache.stopped() {
                if let Err(error) = cache.save(file, bytes) {
                    if !added.is_empty() {
                        changed(added);
                    }

                    return Err(error);
                }

                added.push(file.clone());
            }
        }

        if !added.is_empty() {
            changed(added);
        }

        Ok(())
    }
}
