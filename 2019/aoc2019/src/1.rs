

fn main() {
    let my_str = include_str!("1.in");
    let mut sumfuel = 0.0;
    let data = my_str.split("\n");
    for entry in data {
        let mut entry: f32 = entry.parse().unwrap();
        entry = ((entry / 3.0).floor()) -2.0;
        sumfuel+= entry;
    }
    println!("{}",sumfuel);
}


// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }