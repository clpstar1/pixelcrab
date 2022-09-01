use image::{DynamicImage, GenericImageView};

use crate::c_const::{CELL_SIZE, CELL_SIZE_X, CELL_SIZE_Y};

pub struct IteratorOpts {
    pub threshold: u32,
    pub invert: bool
}

pub struct BrailleCellIterator<'a> {
    img: &'a DynamicImage,
    x: usize,
    y: usize,
    opts: IteratorOpts,
    pub width: usize,
    pub height: usize,
}

impl BrailleCellIterator<'_> {

    pub fn new(img: &DynamicImage, opts: IteratorOpts) -> BrailleCellIterator {
        let w = img.width().try_into().unwrap();
        let h: usize = img.height().try_into().unwrap();
        return BrailleCellIterator { 
            img, 
            x: 0, 
            y: 0,
            opts,
            width: w,
            height: h,
        
        };
    }
}

impl Iterator for BrailleCellIterator<'_> {
    type Item = [u32; CELL_SIZE];

    fn next(&mut self) -> Option<Self::Item> {

        if self.x >= self.width && self.y == self.height {
            return None
        }
        
        let mut res: [u32; CELL_SIZE] = [0; CELL_SIZE];

        for x in 0..CELL_SIZE_X {
            for y in 0..CELL_SIZE_Y {
                let _x: usize = self.x + x;
                let _y: usize = self.y + y;

                if _x > self.width-1 || _y > self.height-1 {
                    return None
                }

                let px = self.img.get_pixel(
                    _x.try_into().unwrap(),
                    _y.try_into().unwrap()
                ).0;
                let [r, g, b, _] = px;
                let lum = ( 
                    u32::from(r) +
                    u32::from(g) + 
                    u32::from(b)
                ) / 3;
                let mut raised = if lum > self.opts.threshold { 0 } else { 1 };

                if self.opts.invert {
                    raised ^= 1;
                }
            
                res[(x*CELL_SIZE_Y) + y] = raised;
            }
        }

        self.x += CELL_SIZE_X;

        if self.x == self.width {
            self.y += CELL_SIZE_Y;
            if self.y != self.height {
                self.x = 0;
            }
        }


        return Some(res)
        
    }
}