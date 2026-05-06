/// wxshadow kernel module FFI bindings
/// 
/// Provides safe Rust interface to wxshadow prctl syscalls

use anyhow::Result;

// prctl options for wxshadow
pub const PR_WXSHADOW_SET_BP: i32 = 0x57580001;
pub const PR_WXSHADOW_SET_REG: i32 = 0x57580002;
pub const PR_WXSHADOW_DEL_BP: i32 = 0x57580003;
pub const PR_WXSHADOW_PATCH: i32 = 0x57580006;
pub const PR_WXSHADOW_RELEASE: i32 = 0x57580008;

/// Set a breakpoint at the given address in target process
pub fn set_breakpoint(pid: i32, addr: u64) -> Result<()> {
    let ret = unsafe {
        libc::prctl(PR_WXSHADOW_SET_BP, pid, addr, 0, 0)
    };
    if ret != 0 {
        anyhow::bail!("Failed to set breakpoint: {}", std::io::Error::last_os_error());
    }
    Ok(())
}

/// Set register value when breakpoint is hit
pub fn set_register(pid: i32, addr: u64, reg_idx: u8, value: u64) -> Result<()> {
    let ret = unsafe {
        libc::prctl(PR_WXSHADOW_SET_REG, pid, addr, reg_idx as usize, value as usize)
    };
    if ret != 0 {
        anyhow::bail!("Failed to set register: {}", std::io::Error::last_os_error());
    }
    Ok(())
}

/// Delete a breakpoint
pub fn delete_breakpoint(pid: i32, addr: u64) -> Result<()> {
    let ret = unsafe {
        libc::prctl(PR_WXSHADOW_DEL_BP, pid, addr, 0, 0)
    };
    if ret != 0 {
        anyhow::bail!("Failed to delete breakpoint: {}", std::io::Error::last_os_error());
    }
    Ok(())
}

/// Write custom patch to shadow page
pub fn write_patch(pid: i32, addr: u64, data: &[u8]) -> Result<()> {
    if data.is_empty() || data.len() > 4096 {
        anyhow::bail!("Invalid patch size: {}", data.len());
    }
    
    let ret = unsafe {
        libc::prctl(
            PR_WXSHADOW_PATCH,
            pid,
            addr,
            data.as_ptr() as usize,
            data.len()
        )
    };
    if ret != 0 {
        anyhow::bail!("Failed to write patch: {}", std::io::Error::last_os_error());
    }
    Ok(())
}

/// Release shadow page
pub fn release_shadow(pid: i32, addr: u64) -> Result<()> {
    let ret = unsafe {
        libc::prctl(PR_WXSHADOW_RELEASE, pid, addr, 0, 0)
    };
    if ret != 0 {
        anyhow::bail!("Failed to release shadow: {}", std::io::Error::last_os_error());
    }
    Ok(())
}

/// ARM64 instruction helpers
pub mod arm64 {
    /// BRK instruction with immediate value
    pub const fn brk(imm: u16) -> u32 {
        0xd4200000 | ((imm as u32) << 5)
    }
    
    /// NOP instruction
    pub const NOP: u32 = 0xd503201f;
    
    /// RET instruction
    pub const RET: u32 = 0xd65f03c0;
    
    /// MOV x0, #imm instruction (limited to 16-bit immediate)
    pub const fn mov_x0_imm(imm: u16) -> u32 {
        0xd2800000 | ((imm as u32) << 5)
    }
    
    /// B (branch) instruction - offset in instructions (not bytes)
    pub const fn b(offset: i32) -> u32 {
        0x14000000 | ((offset as u32) & 0x03ffffff)
    }
    
    /// BL (branch with link) instruction
    pub const fn bl(offset: i32) -> u32 {
        0x94000000 | ((offset as u32) & 0x03ffffff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_arm64_instructions() {
        assert_eq!(arm64::NOP, 0xd503201f);
        assert_eq!(arm64::RET, 0xd65f03c0);
        assert_eq!(arm64::brk(0x7), 0xd42000e0);
    }
}
