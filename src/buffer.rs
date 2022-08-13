use crate::{device::Device, memory::Memory};

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

    fn copy_to_buffer(&self, dst_buffer: &Buffer) {
        //
    }
}

impl<'a> Drop for Buffer<'a> {
    fn drop(&mut self) {}
}
