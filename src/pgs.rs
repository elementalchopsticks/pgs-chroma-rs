use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use std::io::Cursor;

#[derive(Debug)]
struct Pixel {
    y: u8,
    cr: u8,
    cb: u8,
    a: u8,
}

pub fn convert(data: &mut Vec<u8>) -> Result<()> {
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

                let p = convert_pixel(Pixel { y, cr, cb, a });

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

fn convert_pixel(p: Pixel) -> Pixel {
    let y = p.y;
    let cr = 128;
    let cb = 128;
    let a = p.a;

    Pixel { y, cr, cb, a }
}
