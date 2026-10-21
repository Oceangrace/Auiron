use super::game_connection::GameConnection;
use super::game_endpoint::GameEndpoint;
use super::process_handle::ProcessHandle;
use std::mem::{offset_of, size_of};
use std::net::{Ipv4Addr, Ipv6Addr};
use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_NO_MORE_FILES, FILETIME};
use windows::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, MIB_TCP6ROW_OWNER_PID, MIB_TCP6TABLE_OWNER_PID, MIB_TCPROW_OWNER_PID,
    MIB_TCPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_CONNECTIONS,
};
use windows::Win32::Networking::WinSock::{AF_INET, AF_INET6};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{
    GetProcessTimes, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
};

pub(super) struct GameProcess;

impl GameProcess {
    pub(super) fn read(name: &str) -> Result<Option<GameConnection>, String> {
        let Some(pid) = Self::find(name)? else {
            return Ok(None);
        };

        let created_at = Self::created_at(pid);

        let mut endpoints = Self::ipv4(pid)?;

        endpoints.extend(Self::ipv6(pid)?);

        Ok(Some(GameConnection::new(pid, created_at, endpoints)))
    }

    fn created_at(pid: u32) -> Option<u64> {
        // The timestamp strengthens restart detection; endpoint capture requires no process handle.
        // SAFETY: requests query-only access to the discovered process; the guard closes it.
        let process = ProcessHandle::new(
            unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }.ok()?,
        );

        let mut created = FILETIME::default();

        let mut exited = FILETIME::default();

        let mut kernel = FILETIME::default();

        let mut user = FILETIME::default();

        // SAFETY: all four output pointers refer to initialized, writable FILETIME values.
        unsafe {
            GetProcessTimes(
                process.get(),
                &mut created,
                &mut exited,
                &mut kernel,
                &mut user,
            )
        }
        .ok()?;

        Some((u64::from(created.dwHighDateTime) << 32) | u64::from(created.dwLowDateTime))
    }

    fn find(name: &str) -> Result<Option<u32>, String> {
        // SAFETY: process snapshots need no caller-owned pointers.
        let snapshot = ProcessHandle::new(
            unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }
                .map_err(|error| error.to_string())?,
        );

        let mut entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        // SAFETY: entry has the required size and remains writable throughout enumeration.
        let mut result = unsafe { Process32FirstW(snapshot.get(), &mut entry) };

        loop {
            if let Err(error) = result {
                if error.code() == ERROR_NO_MORE_FILES.to_hresult() {
                    return Ok(None);
                }

                return Err(error.to_string());
            }

            let end = entry
                .szExeFile
                .iter()
                .position(|value| *value == 0)
                .unwrap_or(entry.szExeFile.len());

            let executable = String::from_utf16_lossy(&entry.szExeFile[..end]);

            if executable.eq_ignore_ascii_case(name) {
                return Ok(Some(entry.th32ProcessID));
            }

            // SAFETY: same live snapshot and correctly sized output entry.
            result = unsafe { Process32NextW(snapshot.get(), &mut entry) };
        }
    }

    fn table(family: u32) -> Result<Vec<u8>, String> {
        let mut size = 0;

        for _ in 0..4 {
            let mut storage = vec![0u64; (size as usize).div_ceil(8)];

            let capacity = storage.len() * 8;

            let pointer = (!storage.is_empty()).then(|| storage.as_mut_ptr().cast());

            // SAFETY: Windows writes at most `size` bytes; null is the documented size query.
            let result = unsafe {
                GetExtendedTcpTable(
                    pointer,
                    &mut size,
                    false,
                    family,
                    TCP_TABLE_OWNER_PID_CONNECTIONS,
                    0,
                )
            };

            if result == 0 {
                if size as usize > capacity {
                    return Err("Invalid TCP table output size".into());
                }

                // SAFETY: storage is initialized, aligned, and at least `size` bytes long.
                return Ok(unsafe {
                    std::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), size as usize)
                }
                .to_vec());
            }

            if result != ERROR_INSUFFICIENT_BUFFER.0 || size > 16_777_216 {
                return Err(format!(
                    "Cannot read process TCP connections: Windows error {result}"
                ));
            }
        }

        Err("TCP connection table kept changing".into())
    }

    fn ipv4(pid: u32) -> Result<Vec<GameEndpoint>, String> {
        let bytes = Self::table(u32::from(AF_INET.0))?;

        let rows =
            Self::rows::<MIB_TCPROW_OWNER_PID>(&bytes, offset_of!(MIB_TCPTABLE_OWNER_PID, table))?;

        let mut result = Vec::new();

        for row in rows {
            // Include connecting sockets: SYN/ACK can be captured before ESTABLISHED.
            if row.dwOwningPid == pid && row.dwRemotePort != 0 {
                result.push(GameEndpoint::new(
                    Ipv4Addr::from(row.dwLocalAddr.to_ne_bytes()).into(),
                    Ipv4Addr::from(row.dwRemoteAddr.to_ne_bytes()).into(),
                    u16::from_be(row.dwLocalPort as u16),
                    u16::from_be(row.dwRemotePort as u16),
                ));
            }
        }

        Ok(result)
    }

    fn ipv6(pid: u32) -> Result<Vec<GameEndpoint>, String> {
        let bytes = Self::table(u32::from(AF_INET6.0))?;

        let rows = Self::rows::<MIB_TCP6ROW_OWNER_PID>(
            &bytes,
            offset_of!(MIB_TCP6TABLE_OWNER_PID, table),
        )?;

        let mut result = Vec::new();

        for row in rows {
            if row.dwOwningPid == pid && row.dwRemotePort != 0 {
                result.push(GameEndpoint::new(
                    Ipv6Addr::from(row.ucLocalAddr).into(),
                    Ipv6Addr::from(row.ucRemoteAddr).into(),
                    u16::from_be(row.dwLocalPort as u16),
                    u16::from_be(row.dwRemotePort as u16),
                ));
            }
        }

        Ok(result)
    }

    fn rows<T: Copy>(bytes: &[u8], offset: usize) -> Result<Vec<T>, String> {
        let count = u32::from_ne_bytes(
            bytes
                .get(..4)
                .ok_or("Truncated TCP table")?
                .try_into()
                .map_err(|_| "Invalid TCP table")?,
        ) as usize;

        let length = count
            .checked_mul(size_of::<T>())
            .ok_or("TCP table size overflow")?;

        let table = bytes
            .get(
                offset
                    ..offset
                        .checked_add(length)
                        .ok_or("TCP table offset overflow")?,
            )
            .ok_or("Truncated TCP table rows")?;

        // SAFETY: only the two Copy Windows MIB row structs are used; bounds are checked above.
        Ok(table
            .chunks_exact(size_of::<T>())
            .map(|row| unsafe { row.as_ptr().cast::<T>().read_unaligned() })
            .collect())
    }
}
