use image::{DynamicImage, GenericImageView};

const THRESHOLD:u32 = 128;

pub struct DynamicImageWrapper {
    img: DynamicImage,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    
}

impl DynamicImageWrapper {
    pub fn new(img: DynamicImage) -> DynamicImageWrapper {
        let w = img.width().try_into().unwrap();
        let h: usize = img.height().try_into().unwrap();
        return DynamicImageWrapper { 
            img, 
            x: 0, 
            y: 0,
            width: w,
            height: h
        };
    }
}

impl Iterator for DynamicImageWrapper {
    type Item = [u32; 8];

    fn next(&mut self) -> Option<Self::Item> {

        if self.x == self.width && self.y == self.height {
            return None
        }
        
        let mut res: [u32; 8] = [0; 8];

        for x in 0..2usize {
            for y in 0..4usize {
                let px = self.img.get_pixel(
                    u32::try_from(self.x + x).unwrap(),
                    u32::try_from(self.y + y).unwrap()
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