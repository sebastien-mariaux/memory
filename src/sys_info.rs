use sys_info::{mem_info, Error, MemInfo};

pub struct SysInfoWrapper;

impl SysInfoWrapper {
    pub fn new() -> Self {
        SysInfoWrapper
    }

    pub fn get_total_memory(&self) -> u64 {
        let test = mem_info();
        match test {
            Ok(mem) => mem.total,
            Err(_) => panic!("Error getting total memory"),
        }
    }

    pub fn get_available_memory(&self) -> u64 {
        let test = mem_info();
        match test {
            Ok(mem) => mem.avail,
            Err(_) => panic!("Error getting available memory"),
        }
    }
}

impl SysInfoWrapperTrait for SysInfoWrapper {
    fn get_mem_info(&self) -> Result<MemInfo, Error> {
        mem_info()
    }
}

#[cfg_attr(test, mockall::automock)]
pub trait SysInfoWrapperTrait {
    fn get_mem_info(&self) -> Result<sys_info::MemInfo, sys_info::Error>;
}

pub fn get_percent_available_memory(sys_info: &dyn SysInfoWrapperTrait) -> f64 {
    let mem_info = sys_info.get_mem_info().unwrap();
    mem_info.avail as f64 / mem_info.total as f64 * 100.00
}

impl Default for SysInfoWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_percent_available_memory() {
        let sys_info = SysInfoWrapper::new();
        assert!(get_percent_available_memory(&sys_info) > 0.0);
    }

    #[test]
    fn test_get_percent_available_memory_mocked() {
        let mut mock = MockSysInfoWrapperTrait::new();

        // Set expectations and return values
        mock.expect_get_mem_info().returning(|| {
            Ok(sys_info::MemInfo {
                total: 2048,
                avail: 1024,
                free: 1024,
                buffers: 0,
                cached: 0,
                swap_total: 0,
                swap_free: 0,
            })
        });

        // Use the mock in place of the real SysInfoWrapper
        let percent = get_percent_available_memory(&mock);
        assert_eq!(percent, 50.0);
    }
}
