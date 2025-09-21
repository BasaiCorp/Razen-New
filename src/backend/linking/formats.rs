// src/backend/linking/formats.rs

/// Platform-specific executable format utilities
pub struct FormatUtils;

impl FormatUtils {
    /// Get the default executable format for the current platform
    pub fn default_format() -> super::ExecutableFormat {
        #[cfg(target_os = "linux")]
        return super::ExecutableFormat::ELF;
        
        #[cfg(target_os = "windows")]
        return super::ExecutableFormat::PE;
        
        #[cfg(target_os = "macos")]
        return super::ExecutableFormat::MachO;
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        return super::ExecutableFormat::Bytecode;
    }
    
    /// Get the file extension for an executable on the current platform
    pub fn default_extension() -> &'static str {
        #[cfg(target_os = "windows")]
        return ".exe";
        
        #[cfg(not(target_os = "windows"))]
        return "";
    }
    
    /// Check if a format is supported on the current platform
    pub fn is_format_supported(format: super::ExecutableFormat) -> bool {
        match format {
            super::ExecutableFormat::Native => true,
            super::ExecutableFormat::Bytecode => true,
            
            #[cfg(target_os = "linux")]
            super::ExecutableFormat::ELF => true,
            
            #[cfg(target_os = "windows")]
            super::ExecutableFormat::PE => true,
            
            #[cfg(target_os = "macos")]
            super::ExecutableFormat::MachO => true,
            
            _ => false,
        }
    }
}
