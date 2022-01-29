use crate::filter::{Filter, Pixel};

use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use std::io::Cursor;

pub fn convert(data: &mut Vec<u8>, filter: Filter) -> Result<()> {
    let mut buf = Cursor::new(data);

    while !buf.is_empty() {
        let magic = [buf.read_u8()?, buf.read_u8()?];
        buf.set_position(buf.position() + 8);
        let segment_type = buf.read_u8()?;
        let size = buf.read_u16::<BigEndian>()?;

        assert_eq!(magic, [0x50, 0x47]);

        let next_segment = buf.position() + size as u64;

        if segment_type == 0x14 {
            buf.set_position(buf.position() + 2);

            assert_eq!((next_segment - buf.position()) % 5, 0);

            for _ in 0..(next_segment - buf.position()) / 5 {
                let _id = buf.read_u8()?;
                let y = buf.read_u8()?;
                let cr = buf.read_u8()?;
                let cb = buf.read_u8()?;
                let a = buf.read_u8()?;

                let p = Pixel { y, cr, cb, a }.filter(filter);

                buf.set_position(buf.position() - 4);
                buf.write_u8(p.y)?;
                buf.write_u8(p.cr)?;
                buf.write_u8(p.cb)?;
                buf.write_u8(p.a)?;
            }

            assert_eq!(next_segment - buf.position(), 0);
        }

        buf.set_position(next_segment);
    }

    Ok(())
}
