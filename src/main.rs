
use asciilator::ImageASCII;
mod brt;
mod asciilator;
mod lines;
mod files;

fn main() {
    asciilator::DefaultImageASCII.create_ascii_image("C:/Users/lobac/Pictures/shrek4_disneyscreencaps.com_675.0 - Copy.jpg", brt::brt_std);
}
