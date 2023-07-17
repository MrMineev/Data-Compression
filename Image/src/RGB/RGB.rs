pub struct Rgb {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

impl Rgb {
    pub fn new(r: usize, g: usize, b: usize) -> Rgb {
        Rgb {
            r, g, b
        }
    }

    pub fn to_ycbcr(&self) -> (f32, f32, f32) {
        let r: f32 = self.r as f32;
        let g: f32 = self.g as f32;
        let b: f32 = self.b as f32;

        let y = 16.0 + 65.738 * r / 256.0 + 129.057 * g / 256.0 + 25.064 * b / 256.0;
        let cb = 128.0 - 37.945 * r / 256.0 - 74.494 * g / 256.0 + 112.439 * b / 256.0;
        let cr = 128.0 + 112.439 * r / 256.0  - 94.154 * g / 256.0 - 18.285 * b / 256.0;

        return (y, cb, cr);
    }
}
