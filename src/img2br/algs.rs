use rand::{rngs::SmallRng, RngCore, SeedableRng};

pub fn map_pixels(part: [[bool; 2]; 4]) -> char {
    let mut offset: u32 = 0;
    if part[0][0] {
        offset += 0b00000001
    };
    if part[0][1] {
        offset += 0b00001000
    };
    if part[1][0] {
        offset += 0b00000010
    };
    if part[1][1] {
        offset += 0b00010000
    };
    if part[2][0] {
        offset += 0b00000100
    };
    if part[2][1] {
        offset += 0b00100000
    };
    if part[3][0] {
        offset += 0b01000000
    };
    if part[3][1] {
        offset += 0b10000000
    };
    char::from_u32(offset + 0x2800).unwrap()
}
#[derive(Clone, Debug, PartialEq)]
pub struct RandomDeviation {
    color_threshold: u8,
    color_disperce: u8,
}
impl RandomDeviation {
    pub fn new(color_threshold: u8, color_disperce: u8) -> Option<Self> {
        color_threshold.checked_add(color_disperce)?;
        Some(Self {
            color_threshold,
            color_disperce,
        })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum GrayscaleToMono {
    RandomDeviation(RandomDeviation),
}
pub fn img2br(
    image: &image::DynamicImage,
    target_width: u32,
    target_height: u32,
    method: GrayscaleToMono,
) -> String {
    let mut str = "".to_owned();
    let mut r = SmallRng::seed_from_u64(1);
    let img = image.resize(
        target_width,
        target_height,
        image::imageops::FilterType::Triangle,
    );
    let monochrome = img.grayscale().into_luma8();
    for j in 0..((monochrome.height() as f64 / 4.0).ceil() as u32) {
        for i in 0..((monochrome.width() as f64 / 2.0).ceil() as u32) {
            let mut arr: [[bool; 2]; 4] = [[false; 2]; 4];

            for u in 0..4 {
                for t in 0..2 {
                    let pixel = monochrome.get_pixel_checked(i * 2 + t, j * 4 + u);
                    if let Some(p) = pixel {
                        let c = p.0[0];
                        match &method {
                            GrayscaleToMono::RandomDeviation(rd) => {
                                let r_t = if rd.color_disperce > 0 {
                                    (r.next_u32() % rd.color_disperce as u32) as u8
                                } else {
                                    0
                                };
                                if c < rd.color_threshold + r_t {
                                    arr[u as usize][t as usize] = true
                                }
                            }
                        }
                    }
                }
            }
            str += &map_pixels(arr).to_string();
        }
        str += "\n"
    }
    str
}
