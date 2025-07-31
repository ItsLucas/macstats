//! Low-level C FFI bindings for macOS SMC communication

use crate::{
    commands::{smc_key, ReadAction},
    connection::KeyInfo,
    error::{InternalError, InternalResult},
    types::DataValue,
};
#[cfg(target_os = "macos")]
use std::{ffi::CStr, ptr};
use std::{mem::{size_of, size_of_val}, os::raw::c_void};

#[allow(non_camel_case_types)]
type kern_return_t = i32;
#[allow(non_camel_case_types)]
type ipc_port_t = *mut c_void;
#[allow(non_camel_case_types)]
type mach_port_t = ipc_port_t;
#[allow(non_camel_case_types)]
type io_object_t = mach_port_t;
#[allow(non_camel_case_types)]
type io_connect_t = io_object_t;
#[allow(non_camel_case_types)]
type task_t = *mut c_void;
#[allow(non_camel_case_types)]
type task_port_t = task_t;
#[allow(non_camel_case_types)]
type io_service_t = io_object_t;

const MACH_PORT_NULL: mach_port_t = 0 as mach_port_t;
const MASTER_PORT_DEFAULT: mach_port_t = MACH_PORT_NULL;

const KERN_SUCCESS: kern_return_t = 0;
const RETURN_SUCCESS: kern_return_t = KERN_SUCCESS;

const SYS_IOKIT: kern_return_t = (0x38 & 0x3f) << 26;
const SUB_IOKIT_COMMON: kern_return_t = 0;
const RETURN_NOT_PRIVILEGED: kern_return_t = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c1;

const KERNEL_INDEX_SMC: u32 = 2;

#[cfg(target_os = "macos")]
pub(crate) fn num_cpus() -> i32 {
    let mut cpus: i32 = 0;
    let mut cpus_size = size_of_val(&cpus);

    let sysctl_name =
        CStr::from_bytes_with_nul(b"hw.physicalcpu\0").expect("byte literal is missing NUL");

    unsafe {
        if 0 != libc::sysctlbyname(
            sysctl_name.as_ptr(),
            &mut cpus as *mut _ as *mut _,
            &mut cpus_size as *mut _ as *mut _,
            ptr::null_mut(),
            0,
        ) {
            // On ARM targets, processors could be turned off to save power.
            // Use `_SC_NPROCESSORS_CONF` to get the real number.
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            const CONF_NAME: libc::c_int = libc::_SC_NPROCESSORS_CONF;
            #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
            const CONF_NAME: libc::c_int = libc::_SC_NPROCESSORS_ONLN;

            cpus = libc::sysconf(CONF_NAME) as i32;
        }
    }

    cpus.max(1)
}

#[derive(Debug)]
pub(crate) struct SMCConnection {
    conn: io_connect_t,
}

impl Drop for SMCConnection {
    fn drop(&mut self) {
        unsafe { _smc_close(self.conn) }
    }
}

impl SMCConnection {
    pub(crate) fn new() -> InternalResult<Self> {
        let conn = unsafe { _smc_open() }?;
        Ok(Self { conn })
    }

    pub(crate) fn read_value<R>(&mut self, op: R) -> InternalResult<R::Out>
    where
        R: ReadAction,
        R::Out: Default,
    {
        Ok(self.opt_read_value(op)?.unwrap_or_default())
    }

    pub(crate) fn opt_read_value<R: ReadAction>(
        &mut self,
        op: R,
    ) -> InternalResult<Option<R::Out>> {
        let result = self.try_read_value(op);
        match result {
            Ok(result) => Ok(Some(result)),
            Err(InternalError::_UnknownKey) => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn try_read_value<R: ReadAction>(&mut self, op: R) -> InternalResult<R::Out> {
        let key = *op.key();
        let result = unsafe { _smc_read_key(self.conn, key) };
        let result = result.map_err(|e| match e {
            InternalError::_DataKeyError(tpe) => InternalError::DataError { key, tpe },
            otherwise => otherwise,
        })?;
        let tpe = result.data_type;
        let data = &result.bytes.0[..result.data_size as usize];
        let data = DataValue::convert(data, tpe)?;
        op.parse(data).map_err(|e| match e {
            InternalError::_DataValueError => InternalError::DataError { key, tpe },
            otherwise => otherwise,
        })
    }

    pub(crate) fn key_info<O: ReadAction>(&mut self, op: O) -> InternalResult<KeyInfo> {
        let key = *op.key();
        let result = unsafe { _smc_key_info(self.conn, key) };
        result.map_err(|e| match e {
            InternalError::_UnknownKey => InternalError::DataError {
                key,
                tpe: *smc_key(b"????"),
            },
            otherwise => otherwise,
        })
    }

    pub(crate) fn key_info_by_index(&mut self, index: u32) -> InternalResult<KeyInfo> {
        let result = unsafe { _smc_key_index_info(self.conn, index) };
        result.map_err(|e| match e {
            InternalError::_UnknownKey => InternalError::DataError {
                key: *smc_key(b"????"),
                tpe: *smc_key(b"????"),
            },
            otherwise => otherwise,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum SMCReadCommand {
    Data = 5,
    ByIndex = 8,
    KeyInfo = 9,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct SMCKeyData {
    key: u32,
    version: SMCKeyDataVersion,
    p_limit_data: SMCKeyDataLimitData,
    key_info: SMCKeyDataKeyInfo,
    result: u8,
    status: u8,
    data8: u8,
    data32: u32,
    bytes: SMCBytes,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct SMCKeyDataVersion {
    major: u8,
    minor: u8,
    build: u8,
    reserved: u8,
    release: u16,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct SMCKeyDataLimitData {
    version: u16,
    length: u16,
    cpu_p_limit: u32,
    gpu_p_limit: u32,
    mem_p_limit: u32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct SMCKeyDataKeyInfo {
    data_size: u32,
    data_type: u32,
    data_attributes: u8,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct SMCBytes(pub(crate) [u8; 32]);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct SMCVal {
    key: u32,
    data_size: u32,
    data_type: u32,
    bytes: SMCBytes,
}

#[repr(C)]
struct __CFDictionary(c_void);

type CFDictionaryRef = *const __CFDictionary;
type CFMutableDictionaryRef = *mut __CFDictionary;

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOServiceMatching(name: *const u8) -> CFMutableDictionaryRef;

    fn IOServiceGetMatchingService(
        master_port: mach_port_t,
        matching: CFDictionaryRef,
    ) -> io_service_t;

    fn IOServiceOpen(
        service: io_service_t,
        owning_task: task_port_t,
        r#type: u32,
        connect: *const io_connect_t,
    ) -> kern_return_t;

    fn IOServiceClose(connect: io_connect_t) -> kern_return_t;

    fn IOConnectCallStructMethod(
        connection: mach_port_t,
        selector: u32,
        input: *const c_void,
        input_size: usize,
        output: *mut c_void,
        output_size: *mut usize,
    ) -> kern_return_t;

    fn IOObjectRelease(object: io_object_t) -> kern_return_t;

    fn mach_task_self() -> mach_port_t;
}

unsafe fn _smc_open() -> InternalResult<io_connect_t> {
    let matching_dictionary = IOServiceMatching(b"AppleSMC\0".as_ptr());
    let device = IOServiceGetMatchingService(MASTER_PORT_DEFAULT, matching_dictionary);

    if device.is_null() {
        return Err(InternalError::SmcNotFound);
    }

    let result: kern_return_t;
    let conn: io_connect_t = MASTER_PORT_DEFAULT;
    result = IOServiceOpen(device, mach_task_self(), 0, &conn);
    let _ = IOObjectRelease(device);

    if result != RETURN_SUCCESS {
        return Err(InternalError::SmcFailedToOpen(result));
    }

    Ok(conn)
}

unsafe fn _smc_close(conn: io_connect_t) {
    let _ = IOServiceClose(conn);
}

unsafe fn _smc_read_key(conn: mach_port_t, key: u32) -> InternalResult<SMCVal> {
    let mut input = SMCKeyData::default();
    input.key = key;
    input.data8 = SMCReadCommand::KeyInfo as u8;

    let mut output = SMCKeyData::default();
    _smc_call(conn, &input, &mut output)?;

    let data_type = output.key_info.data_type;
    let data_size = output.key_info.data_size;

    if data_size > 32 {
        return Err(InternalError::_DataKeyError(data_type));
    }

    input.key_info.data_size = data_size;
    input.data8 = SMCReadCommand::Data as u8;

    _smc_call(conn, &input, &mut output)?;

    let val = SMCVal {
        key,
        data_size,
        data_type,
        bytes: output.bytes,
    };

    Ok(val)
}

unsafe fn _smc_key_info(conn: mach_port_t, key: u32) -> InternalResult<KeyInfo> {
    let mut input = SMCKeyData::default();
    input.key = key;
    input.data8 = SMCReadCommand::KeyInfo as u8;

    let mut output = SMCKeyData::default();
    _smc_call(conn, &input, &mut output)?;

    let data_type = output.key_info.data_type;
    let data_size = output.key_info.data_size;

    Ok(KeyInfo {
        key,
        data_type,
        data_size,
    })
}

unsafe fn _smc_key_index_info(conn: mach_port_t, index: u32) -> InternalResult<KeyInfo> {
    let mut input = SMCKeyData::default();
    input.data8 = SMCReadCommand::ByIndex as u8;
    input.data32 = index;

    let mut output = SMCKeyData::default();
    _smc_call(conn, &input, &mut output)?;

    let key = output.key;
    let data_type = output.key_info.data_type;
    let data_size = output.key_info.data_size;

    Ok(KeyInfo {
        key,
        data_type,
        data_size,
    })
}

unsafe fn _smc_call(
    conn: mach_port_t,
    input: &SMCKeyData,
    output: &mut SMCKeyData,
) -> InternalResult<()> {
    let mut output_size = size_of::<SMCKeyData>();

    let result = IOConnectCallStructMethod(
        conn,
        KERNEL_INDEX_SMC,
        input as *const _ as *const c_void,
        size_of::<SMCKeyData>(),
        output as *mut _ as *mut c_void,
        &mut output_size,
    );

    if result == RETURN_NOT_PRIVILEGED {
        return Err(InternalError::NotPrivlileged);
    }
    if result != RETURN_SUCCESS {
        return Err(InternalError::UnknownSmc(result, output.result));
    }
    if output.result == 132 {
        return Err(InternalError::_UnknownKey);
    }

    Ok(())
}