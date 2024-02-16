fn main() {
    let width: u64 = 256;
    let height: u64 = 256;

    println!("P3");
    println!("{:}", width);
    println!("{:}", height);
    println!("255");

    for y in 0..height {
        for x in 0..width {
            eprint!("\rx:{:5}, y:{:5}", x, y);

            let r = y as f64 / ((width - 1) as f64) * 255.99;
            let g = x as f64 / ((height - 1) as f64) * 255.99;
            let b = 0.25 * 255.99;

            println!("{:.0} {:.0} {:.0}", r, g, b)
        }
    }
    eprintln!();
}
