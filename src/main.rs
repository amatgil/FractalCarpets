
use ppmitzador::{self, ImagePBM, PpmFormat};

/// Note that they must have the EXACT SAME dimensions
struct Carpet {
    axiom: ImagePBM,
    gen: ImagePBM,
}

// TODO: generalize
/// Scale up A SQUARE ONLY
fn scale(original: &ImagePBM, scale: usize) -> ImagePBM {
    let mut new_img = ImagePBM::new(original.width()*scale, original.height()*scale, true);
    let x_ratio = original.width() as f32 / new_img.width() as f32;
    let y_ratio = original.height() as f32 / new_img.height() as f32;

    for y in 0..new_img.height() {
        for x in 0..new_img.width() {
            let nearest_x = ((x as f32 * x_ratio).round() as usize).min(original.width() - 1);
            let nearest_y = ((y as f32 * y_ratio).round() as usize).min(original.height() - 1);

            dbg!(x, y, nearest_x, nearest_y);
            *new_img.get_mut(x, y).unwrap() = *original.get(nearest_x, nearest_y).unwrap();
        }
    }

    new_img
}

fn generate_carpet(Carpet { axiom, gen }: Carpet, n: usize) -> ImagePBM {
    fn go(canvas: &mut ImagePBM, ox: usize, oy: usize, n: usize, gen: &ImagePBM) {
        let d = gen.width();

        for y in oy..oy+d {
            for x in ox..ox+d {
                // apply it
            }
        }
    }
    let mut base = scale(&axiom, n.pow(axiom.width() as u32));

    // Do vvvvv for each pixel
    go(&mut base, ox, oy, n, &gen);
    // Do ^^^^^ for each pixel

    todo!()
}

fn main() {
    let mut sierp_carpet_gen = ImagePBM::new(3, 3, true);
    *sierp_carpet_gen.get_mut(1, 1).unwrap() = false;
    let sierp_carpet = Carpet {
        axiom: ImagePBM::new(3, 3, true),
        gen: sierp_carpet_gen,
    };

    let mut jerusalem_gen = ImagePBM::new(7, 7, true);
    for y in 1..=5 { *jerusalem_gen.get_mut(3, y).unwrap() = false; }
    for x in 1..=5 { *jerusalem_gen.get_mut(x, 3).unwrap() = false; }
    let jerusalem_carpet = Carpet {
        axiom: ImagePBM::new(7, 7, true),
        gen: jerusalem_gen.clone(),
    };


    let scale_test = scale(&jerusalem_gen, 10);
    jerusalem_gen.save_to_file("original.pbm").unwrap();
    scale_test.save_to_file("scaled.pbm").unwrap();


    let c = generate_carpet(sierp_carpet, 3);
    c.save_to_file("sierp_carpet.pbm").unwrap();


}
