use image::{DynamicImage, GenericImageView};

const THRESHOLD:u32 = 128;

pub struct DynamicImageWrapper {
    img: DynamicImage,
    x: usize,
    y: usize,
    pub width: usize,
    pub height: usize,
    pub threshold: u32
}

impl DynamicImageWrapper {

    pub fn new(img: DynamicImage, threshold: u32) -> DynamicImageWrapper {
        let w = img.width().try_into().unwrap();
        let h: usize = img.height().try_into().unwrap();
        return DynamicImageWrapper { 
            img, 
            x: 0, 
            y: 0,
            width: w,
            height: h,
            threshold
        };
    }
}

impl Iterator for DynamicImageWrapper {
    type Item = [u32; 8];

    fn next(&mut self) -> Option<Self::Item> {

        if self.x >= self.width && self.y == self.height {
            return None
        }
        
        let mut res: [u32; 8] = [0; 8];

        for x in 0..2usize {
            for y in 0..4usize {
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
                let lum =( 
                    u32::from(r) +
                    u32::from(g) + 
                    u32::from(b)
                ) / 3;
                let raised = if lum > THRESHOLD { 0 } else { 1 };
                res[(x*4) + y] = raised;
            }
        }

        self.x += 2;

        if self.x == self.width {
            self.y += 4;
            if self.y != self.height {
                self.x = 0;
            }
        }


        return Some(res)
        
    }
}