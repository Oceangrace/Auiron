use std::marker::PhantomData;
use std::rc::Rc;
use windows::Win32::System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize, RoUninitialize};

pub(super) struct RuntimeApartment {
    thread: PhantomData<Rc<()>>,
}

impl RuntimeApartment {
    pub(super) fn enter() -> Result<Self, String> {
        // Each blocking worker owns its successful WinRT initialization.
        unsafe { RoInitialize(RO_INIT_MULTITHREADED) }
            .map_err(|error| format!("Could not initialize Windows speech: {error}"))?;

        Ok(Self {
            thread: PhantomData,
        })
    }
}

impl Drop for RuntimeApartment {
    fn drop(&mut self) {
        // Balanced on the same worker thread, after all WinRT objects are dropped.
        unsafe { RoUninitialize() };
    }
}
