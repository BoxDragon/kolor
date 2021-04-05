use kolor::{spaces, Color};

pub fn example() {
    let srgb = Color::new(0.35, 0.75, 0.8, spaces::SRGB);
    let mut oklab = srgb.to(spaces::OKLAB);
    // modify `a`
    oklab.value.y += 0.2;
    let modified_srgb = oklab.to(srgb.space);
    println!(" {:?} -> {:?}", srgb, modified_srgb);
}
