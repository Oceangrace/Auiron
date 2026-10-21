use super::StreamKey;
use super::TcpSegment;
use crate::integrations::aion2::capture::CapturedPacket;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub(crate) struct PacketReader;

impl PacketReader {
    pub(crate) fn read(packet: &CapturedPacket) -> Result<Option<TcpSegment>, String> {
        let offset = match packet.link_type() {
            0 | 108 => 4,
            101 | 228 | 229 => 0,
            1 => match Self::ethernet(packet.data())? {
                Some(value) => value,
                None => return Ok(None),
            },
            _ => return Ok(None),
        };

        let ip = packet.data().get(offset..).ok_or("Truncated link header")?;

        match ip.first().map(|byte| byte >> 4) {
            Some(4) => Self::ipv4(ip, packet.timestamp_us()),
            Some(6) => Self::ipv6(ip, packet.timestamp_us()),
            _ => Ok(None),
        }
    }

    fn ethernet(bytes: &[u8]) -> Result<Option<usize>, String> {
        if bytes.len() < 14 {
            return Err("Truncated Ethernet header".into());
        }

        let mut offset = 14;

        let mut protocol = Self::u16(&bytes[12..14])?;

        for _ in 0..2 {
            if !matches!(protocol, 0x8100 | 0x88a8) {
                break;
            }

            let tag = bytes
                .get(offset..offset + 4)
                .ok_or("Truncated VLAN header")?;

            protocol = Self::u16(&tag[2..])?;
            offset += 4;
        }

        Ok(matches!(protocol, 0x0800 | 0x86dd).then_some(offset))
    }

    fn ipv4(ip: &[u8], at: u64) -> Result<Option<TcpSegment>, String> {
        if ip.len() < 20 || ip[9] != 6 {
            return Ok(None);
        }

        let size = usize::from(Self::u16(&ip[2..4])?);

        let header = usize::from(ip[0] & 15) * 4;

        if size > ip.len() || header < 20 || size < header + 20 {
            return Err("Truncated IPv4 capture".into());
        }

        if Self::u16(&ip[6..8])? & 0x3fff != 0 {
            return Ok(None);
        }

        let source = Ipv4Addr::new(ip[12], ip[13], ip[14], ip[15]);

        let destination = Ipv4Addr::new(ip[16], ip[17], ip[18], ip[19]);

        Self::tcp(&ip[header..size], source.into(), destination.into(), at).map(Some)
    }

    fn ipv6(ip: &[u8], at: u64) -> Result<Option<TcpSegment>, String> {
        if ip.len() < 40 {
            return Err("Truncated IPv6 header".into());
        }

        let end = 40 + usize::from(Self::u16(&ip[4..6])?);

        if end > ip.len() {
            return Err("Truncated IPv6 capture".into());
        }

        let source =
            Ipv6Addr::from(<[u8; 16]>::try_from(&ip[8..24]).map_err(|_| "Invalid IPv6 source")?);

        let destination = Ipv6Addr::from(
            <[u8; 16]>::try_from(&ip[24..40]).map_err(|_| "Invalid IPv6 destination")?,
        );

        let mut offset = 40;

        let mut next = ip[6];

        for _ in 0..8 {
            if next == 6 {
                return Self::tcp(&ip[offset..end], source.into(), destination.into(), at)
                    .map(Some);
            }

            if !matches!(next, 0 | 43 | 60) {
                return Ok(None);
            }

            let header = ip
                .get(offset..offset + 2)
                .filter(|_| offset + 2 <= end)
                .ok_or("Truncated IPv6 extension")?;

            let size = (usize::from(header[1]) + 1) * 8;

            if size > end - offset {
                return Err("Truncated IPv6 extension".into());
            }

            next = header[0];
            offset += size;
        }

        Ok(None)
    }

    fn tcp(tcp: &[u8], source: IpAddr, destination: IpAddr, at: u64) -> Result<TcpSegment, String> {
        if tcp.len() < 20 {
            return Err("Truncated TCP header".into());
        }

        let header = usize::from(tcp[12] >> 4) * 4;

        if header < 20 || header > tcp.len() {
            return Err("Invalid TCP header".into());
        }

        let source_port = Self::u16(&tcp[..2])?;

        let destination_port = Self::u16(&tcp[2..4])?;

        let key = StreamKey::new(source, destination, source_port, destination_port);

        let sequence =
            u32::from_be_bytes(tcp[4..8].try_into().map_err(|_| "Invalid TCP sequence")?);

        Ok(TcpSegment::new(
            key,
            source_port,
            sequence,
            tcp[header..].to_vec(),
            at,
            tcp[13] & 7 != 0,
        )
        .with_finished(tcp[13] & 1 != 0))
    }

    fn u16(bytes: &[u8]) -> Result<u16, String> {
        Ok(u16::from_be_bytes(
            bytes.try_into().map_err(|_| "Invalid network integer")?,
        ))
    }
}
