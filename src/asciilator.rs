use std::{ffi::OsString, future, path::PathBuf, str::FromStr, thread::current};

use image::{GenericImageView, Pixel, Rgb};

#[macro_export]
macro_rules! check {
    ($temp: expr => $true: expr, $false: expr) => {
        if $temp {
            $true
        } else {
            $false
        }
    };
}

pub trait ImageASCII {
    fn create_ascii_image(&self, path: &str, brt: fn(&[u8]) -> f32) -> String;
}
pub struct DefaultImageASCII;

impl ImageASCII for DefaultImageASCII {
    fn create_ascii_image(&self, path: &str, brt: fn(&[u8]) -> f32) -> String{
        let path = PathBuf::from_str(path).unwrap();
        let mut result = String::new();
        if std::fs::metadata(&path).is_ok() {
            let mut _image = image::open(path).unwrap();

            let width: u32 = _image.width();
            let height: u32 = _image.height();
            let mut drt_buff = vec![vec![0.0; width as usize]; height as usize];
            let mut curr_avg = 1.0;
            let mut cnt = 0;

            for i in 0..height {
                for j in 0..width {
                    let pix = _image.get_pixel(j, i).to_rgb();
                    let channels = pix.channels();

                    let pxc_brt = brt(channels);
                    calculate_avg(&mut curr_avg, &mut cnt, pxc_brt);

                    drt_buff[i as usize][j as usize] = pxc_brt;
                }
            }

            for i in 0..drt_buff.len() {
                for j in 0..drt_buff[0].len() {
                    let val = drt_buff[i][j] as f64;

                    result += check!(val >= curr_avg => "%", ".");
                }
                result += "\n";
            }
        }
        result
    }
}

fn calculate_avg(avg: &mut f64, cnt: &mut u64, new_val: f32) {
    *avg = ((*avg * (*cnt as f64)) + new_val as f64) / (*cnt + 1) as f64;
    *cnt += 1;
}
