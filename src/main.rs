pub struct Device<'a> {
    id: &'a u32,
}

impl<'a> Device<'a> {
    pub fn new(id: &'a u32) -> Self {
        Self { id }
    }
}

pub struct Memory<'a> {
    device: &'a Device<'a>,
}

impl<'a> Memory<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self { device }
    }

    pub fn map(&mut self) {}

    pub fn unmap(&mut self) {}

    pub fn copy_from_host<T>(&self, _slice: &[T]) {}
}

impl<'a> Drop for Memory<'a> {
    fn drop(&mut self) {}
}

pub struct Buffer<'a> {
    device: &'a Device<'a>,
    memory: Memory<'a>,
}

impl<'a> Buffer<'a> {
    pub fn new(device: &'a Device) -> Self {
        let buffer_memory = Memory::new(device);

        Self {
            device,
            memory: buffer_memory,
        }
    }

    pub fn from_data<T: Copy>(device: &'a Device, data: &[T]) -> Self {
        let mut staging_buffer = Buffer::new(device);

        {
            let staging_buffer_memory = staging_buffer.memory_mut();
            staging_buffer_memory.map();
            staging_buffer_memory.copy_from_host(data);
            staging_buffer_memory.unmap();
        }

        let buffer = Self::new(device);
        staging_buffer.copy_to_buffer(&buffer);

        buffer
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn memory_mut(&'a mut self) -> &mut Memory {
        &mut self.memory
    }

    fn copy_to_buffer(&self, _dst_buffer: &Buffer) {
        //
    }
}

impl<'a> Drop for Buffer<'a> {
    fn drop(&mut self) {}
}

fn main() {
    let id = 5;
    let device = Device::new(&id);
    let _buffer = Buffer::new(&device);
}
