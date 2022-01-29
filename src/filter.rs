#[derive(Copy, Clone)]
pub enum Filter {
    Grayscale,
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub y: u8,
    pub cr: u8,
    pub cb: u8,
    pub a: u8,
}

impl Pixel {
    pub fn filter(self, filter: Filter) -> Self {
        match filter {
            Filter::Grayscale => greyscale(self),
        }
    }
}

fn greyscale(p: Pixel) -> Pixel {
    let y = p.y;
    let cr = 128;
    let cb = 128;
    let a = p.a;

    Pixel { y, cr, cb, a }
}
