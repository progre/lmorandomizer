use std::mem::size_of;

use anyhow::{Result, bail};
use windows::{
    Win32::{
        Foundation::HANDLE,
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_SHARE_READ, OPEN_EXISTING,
        },
        System::{
            Diagnostics::{
                Debug::{
                    IMAGE_DIRECTORY_ENTRY_EXPORT, ImageDirectoryEntryToDataEx, ImageNtHeader,
                    ImageRvaToVa,
                },
                ToolHelp::{
                    CreateToolhelp32Snapshot, MODULEENTRY32W, Module32FirstW, Module32NextW,
                    TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
                },
            },
            Memory::{
                CreateFileMappingW, FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile,
                PAGE_READONLY, UnmapViewOfFile,
            },
            SystemServices::IMAGE_EXPORT_DIRECTORY,
        },
    },
    core::{Owned, PCSTR, PCWSTR},
};

fn module_entry(process_id: u32, module_name: &str) -> Result<MODULEENTRY32W> {
    let flags = TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32;
    let snapshot = unsafe { Owned::new(CreateToolhelp32Snapshot(flags, process_id)?) };

    let mut me = MODULEENTRY32W {
        dwSize: size_of::<MODULEENTRY32W>() as u32,
        ..Default::default()
    };

    unsafe { Module32FirstW(*snapshot, &mut me) }?;
    loop {
        let name = unsafe { PCWSTR::from_raw(me.szModule.as_ptr()).to_string() }?;
        if name.eq_ignore_ascii_case(module_name) {
            return Ok(me);
        }

        unsafe { Module32NextW(*snapshot, &mut me) }?;
    }
}

fn addr_from_map_view_of_file(base: &ViewOfFile, func_name: &str) -> Result<usize> {
    let nt_hdrs = unsafe { ImageNtHeader(base.0.Value) };
    let mut exp_size: u32 = Default::default();
    let exp = unsafe {
        ImageDirectoryEntryToDataEx(
            base.0.Value,
            false,
            IMAGE_DIRECTORY_ENTRY_EXPORT,
            &mut exp_size,
            None,
        )
    } as *const IMAGE_EXPORT_DIRECTORY;
    if exp.is_null() {
        bail!("ImageDirectoryEntryToDataEx failed");
    }
    let exp = unsafe { *exp };
    if exp.NumberOfNames == 0 {
        bail!("NumberOfNames is 0");
    }
    let addr_of_names =
        unsafe { ImageRvaToVa(nt_hdrs, base.0.Value, exp.AddressOfNames, None) } as *const u32;
    if addr_of_names.is_null() {
        bail!("ImageRvaToVa failed");
    }
    let addr_of_name_ordinals =
        unsafe { ImageRvaToVa(nt_hdrs, base.0.Value, exp.AddressOfNameOrdinals, None) }
            as *const u16;
    if addr_of_name_ordinals.is_null() {
        bail!("ImageRvaToVa failed");
    }
    let addr_of_funcs =
        unsafe { ImageRvaToVa(nt_hdrs, base.0.Value, exp.AddressOfFunctions, None) } as *const u32;
    if addr_of_funcs.is_null() {
        bail!("ImageRvaToVa failed");
    }

    let idx = (0..exp.NumberOfNames as usize)
        .map(
            |x| unsafe { ImageRvaToVa(nt_hdrs, base.0.Value, *addr_of_names.add(x), None) }
                as *const u8,
        )
        .position(|x| unsafe { PCSTR::from_raw(x).to_string() }.unwrap() == func_name)
        .unwrap();

    Ok((unsafe { *(addr_of_funcs.add(*(addr_of_name_ordinals.add(idx)) as usize)) }) as usize)
}

struct ViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS);

impl ViewOfFile {
    pub fn map(file_mapping: HANDLE) -> Result<Self> {
        let view = unsafe { MapViewOfFile(file_mapping, FILE_MAP_READ, 0, 0, 0) };
        if view.Value.is_null() {
            bail!("MapViewOfFile failed");
        }
        Ok(Self(view))
    }
}

impl Drop for ViewOfFile {
    fn drop(&mut self) {
        unsafe { UnmapViewOfFile(self.0) }.unwrap();
    }
}

pub fn find_func_addr(process_id: u32, module_name: &str, func_name: &str) -> Result<usize> {
    let me = module_entry(process_id, module_name)?;

    let file = unsafe {
        Owned::new(CreateFileW(
            PCWSTR::from_raw(me.szExePath.as_ptr()),
            FILE_GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )?)
    };

    let file_mapping =
        unsafe { Owned::new(CreateFileMappingW(*file, None, PAGE_READONLY, 0, 0, None)?) };
    let base = ViewOfFile::map(*file_mapping)?;

    Ok(me.modBaseAddr as usize + addr_from_map_view_of_file(&base, func_name)?)
}
