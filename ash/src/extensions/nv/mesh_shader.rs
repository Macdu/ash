//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_mesh_shader.html>

use crate::vk;
use core::ffi;
use core::mem;

pub const NAME: &ffi::CStr = vk::nv::mesh_shader::NAME;

#[derive(Clone)]
pub struct Device {
    fp: vk::nv::mesh_shader::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let fp = vk::nv::mesh_shader::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksNV.html>
    #[inline]
    pub unsafe fn cmd_draw_mesh_tasks(
        &self,
        command_buffer: vk::CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) {
        (self.fp.cmd_draw_mesh_tasks_nv)(command_buffer, task_count, first_task);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectNV.html>
    #[inline]
    pub unsafe fn cmd_draw_mesh_tasks_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self.fp.cmd_draw_mesh_tasks_indirect_nv)(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDrawMeshTasksIndirectCountNV.html>
    #[inline]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.fp.cmd_draw_mesh_tasks_indirect_count_nv)(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }

    #[inline]
    pub fn fp(&self) -> &vk::nv::mesh_shader::DeviceFn {
        &self.fp
    }
}
