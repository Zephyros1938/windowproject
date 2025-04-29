use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub mod glmaddon;

pub fn init_logging() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::debug!("log4rs configured!");
}

pub enum LinuxExitCode {
    OK,
    ERR(u8),
}

impl std::process::Termination for LinuxExitCode {
    fn report(self) -> std::process::ExitCode {
        match self {
            LinuxExitCode::OK => std::process::ExitCode::SUCCESS,
            LinuxExitCode::ERR(v) => std::process::ExitCode::from(v),
        }
    }
}

pub struct BinaryReader {
    position: u64,
    f: File,
}

impl BinaryReader {
    pub fn new(file: File) -> BinaryReader {
        Self {
            position: 0,
            f: file,
        }
    }

    pub fn get_uint8(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.f.seek(SeekFrom::Start(self.position)).unwrap();
        self.f.read_exact(&mut buf).unwrap();
        self.position += 1;
        buf[0]
    }

    pub fn get_uint16(&mut self) -> u16 {
        let hi = self.get_uint8() as u16;
        let lo = self.get_uint8() as u16;
        (hi << 8) | lo
    }

    pub fn get_uint32(&mut self) -> u32 {
        let b1 = self.get_uint8() as u32;
        let b2 = self.get_uint8() as u32;
        let b3 = self.get_uint8() as u32;
        let b4 = self.get_uint8() as u32;
        (b1 << 24) | (b2 << 16) | (b3 << 8) | b4
    }

    pub fn get_int16(&mut self) -> i16 {
        self.get_uint16() as i16
    }

    pub fn get_int32(&mut self) -> i32 {
        let b1 = self.get_uint8() as u32;
        let b2 = self.get_uint8() as u32;
        let b3 = self.get_uint8() as u32;
        let b4 = self.get_uint8() as u32;
        ((b1 << 24) | (b2 << 16) | (b3 << 8) | b4) as i32
    }

    pub fn get_fword(&mut self) -> i16 {
        self.get_int16()
    }

    pub fn get_ufword(&mut self) -> i32 {
        self.get_int32()
    }

    pub fn get_offset16(&mut self) -> u16 {
        self.get_uint16()
    }

    pub fn get_offset32(&mut self) -> u32 {
        self.get_uint32()
    }

    pub fn get_f2dot14(&mut self) -> i16 {
        self.get_int16() / (1 << 14)
    }

    pub fn get_fixed(&mut self) -> i32 {
        self.get_int32() / (1 << 16)
    }

    pub fn get_string(&mut self, length: usize) -> String {
        let mut buf = vec![0u8; length];
        self.f.seek(SeekFrom::Start(self.position)).unwrap();
        self.f.read_exact(&mut buf).unwrap();
        self.position += length as u64;
        String::from_utf8_lossy(&buf).into_owned()
    }

    pub fn get_date(&mut self) -> SystemTime {
        let high = self.get_uint32() as u64;
        let low = self.get_uint32() as u64;
        let mac_time = (high << 32) | low;

        // Mac epoch starts at 1904-01-01 UTC; UNIX epoch is 1970-01-01 UTC
        const SECONDS_FROM_1904_TO_1970: u64 = 2082844800;

        // Convert Mac time to Unix time
        let unix_time = mac_time.checked_sub(SECONDS_FROM_1904_TO_1970).unwrap_or(0);

        UNIX_EPOCH + Duration::from_secs(unix_time)
    }

    pub fn get_position(&self) -> u64 {
        self.position
    }

    pub fn set_position(&mut self, position: u64) {
        self.position = position
    }
}
