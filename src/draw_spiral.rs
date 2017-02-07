extern crate image;

use self::image::ImageBuffer;

use std::fs::File;
use std::path::PathBuf;


pub fn prime_filter_to_spiral_png(x_size: usize, from_vec: Vec<bool>, path: String){
    let imgbuf = make_spiral_buffer(x_size, from_vec);
    let mut w_path = PathBuf::from(path);

    w_path.push(format!("ulam_spiral{}", x_size));
    w_path.set_extension("png");

    let ref mut four = File::create(w_path).unwrap();

    let _ = image::ImageLuma8(imgbuf).save(four, image::PNG);
}
fn point_on_spiral_to_array_index_end(spiral_size: usize, x: usize, y: usize)->usize{
    let ss_minus_one = spiral_size-1;
    match (x, y) {
        (x, y) if y == 0 => x,
        (x, y) if x == 0 => ss_minus_one*4 - y,
        (x, y) if y == spiral_size-1 => ss_minus_one*3 - x,
        (x, y) if x == spiral_size-1 => ss_minus_one + y,
        (x, y) => ss_minus_one*4 + point_on_spiral_to_array_index_end(spiral_size-2, x-1, y-1),
    }

}
fn point_on_spiral_to_array_index(spiral_size: usize, x: usize, y: usize)->usize{
    spiral_size*spiral_size - point_on_spiral_to_array_index_end(spiral_size, x, y)
}
fn make_spiral_buffer(x_size: usize, from_vec: Vec<bool>)->
                    ImageBuffer<image::Luma<u8>, Vec<u8>>{
    assert_eq!(from_vec.len(), x_size*x_size+1);
    let img = ImageBuffer::from_fn(x_size as u32, x_size as u32, |x, y| {
        match from_vec[point_on_spiral_to_array_index(x_size, x as usize, y as usize)] {
            true  => image::Luma([0u8]),
            false => image::Luma([255u8]),
        }
    });
    img
}
