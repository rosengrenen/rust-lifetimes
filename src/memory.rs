use crate::device::Device;

pub struct Memory<'a> {
    device: &'a Device<'a>,
    is_mapped: bool,
}

impl<'a> Memory<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            is_mapped: false,
        }
    }

    pub fn map(&mut self) {
        if self.is_mapped {
            println!("WARNING: memory is already mapped");
            return;
        }

        self.is_mapped = true;
    }

    pub fn unmap(&mut self) {
        if !self.is_mapped {
            println!("WARNING: memory is already unmapped");
            return;
        }

        self.is_mapped = false;
    }

    pub fn copy_from_host<T>(&self, _slice: &[T]) {
        if self.is_mapped {
            //
        } else {
            panic!("memory not mapped");
        }
    }
}

impl<'a> Drop for Memory<'a> {
    fn drop(&mut self) {}
}
