use std::ops::Range;

use crate::geometry::Vertex;

/// Vertex buffer that grows to fit whatever the frame needs and is reused
/// across frames otherwise.
pub(super) struct DynamicVertexBuffer {
    buffer: wgpu::Buffer,
    capacity: u64,
    pub(super) len: u32,
    label: &'static str,
}

impl DynamicVertexBuffer {
    const INITIAL_CAPACITY: u64 = 4096;

    pub(super) fn new(device: &wgpu::Device, label: &'static str) -> Self {
        Self {
            buffer: Self::allocate(device, label, Self::INITIAL_CAPACITY),
            capacity: Self::INITIAL_CAPACITY,
            len: 0,
            label,
        }
    }

    fn allocate(device: &wgpu::Device, label: &'static str, capacity: u64) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: capacity * size_of::<Vertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub(super) fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[Vertex],
    ) {
        self.len = vertices.len() as u32;
        if vertices.is_empty() {
            return;
        }
        if vertices.len() as u64 > self.capacity {
            self.capacity = (vertices.len() as u64).next_power_of_two();
            self.buffer = Self::allocate(device, self.label, self.capacity);
        }
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(vertices));
    }

    /// Draws the buffer as triangles, one vertex each.
    pub(super) fn draw_triangles(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.draw_triangles_in(pass, 0..self.len);
    }

    pub(super) fn draw_triangles_in(&self, pass: &mut wgpu::RenderPass<'_>, vertices: Range<u32>) {
        if vertices.is_empty() {
            return;
        }
        pass.set_vertex_buffer(0, self.buffer.slice(..));
        pass.draw(vertices, 0..1);
    }

    /// Draws the buffer as lines: every pair of vertices is one instance, and
    /// the shader expands it into a six-vertex quad.
    pub(super) fn draw_lines(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.len < 2 {
            return;
        }
        pass.set_vertex_buffer(0, self.buffer.slice(..));
        pass.draw(0..6, 0..self.len / 2);
    }
}
