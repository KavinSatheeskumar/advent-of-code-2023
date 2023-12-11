use std::fs;
use regex::Regex;

fn parse_line(s : &str) -> (i32, Vec<(i32, i32, i32)>) {
    let regex_game_num = Regex::new(r"Game ([0-9]+): (.+)").unwrap();

    let (_, [nums, g]) = regex_game_num.captures_iter(s).map(|c| c.extract()).next().unwrap();
    let game_num : i32 = nums.parse::<i32>().unwrap();
    let games : &str = g;

    let draws : Vec<&str> = games.split(";").collect();
    let mut out_vec = Vec::<(i32, i32, i32)>::new();

    for d in draws {
        let mut red : i32 = 0;
        let mut blue : i32 = 0;
        let mut green : i32 = 0;

        let regex_game_red = Regex::new(r"([0-9]+) red").unwrap();
        let regex_game_blue = Regex::new(r"([0-9]+) blue").unwrap();
        let regex_game_green = Regex::new(r"([0-9]+) green").unwrap();

        match regex_game_red.captures_iter(d).map(|c| c.extract()).next() {
            Some((_, [num_red])) => red = num_red.parse::<i32>().unwrap(),
            None => ()
        };

        match regex_game_blue.captures_iter(d).map(|c| c.extract()).next() {
            Some((_, [num_blue])) => blue = num_blue.parse::<i32>().unwrap(),
            None => ()
        };

        match regex_game_green.captures_iter(d).map(|c| c.extract()).next() {
            Some((_, [num_green])) => green = num_green.parse::<i32>().unwrap(),
            None => ()
        };

        out_vec.push((red, green, blue));

    }

    return (game_num, out_vec);

}

fn main() {
    let contents: String = fs::read_to_string("./inp.in")
        .expect("Should have been able to read the file");

    let lines = contents.split('\n');

    let mut output_part_1 = 0;
    let mut output_part_2 = 0;

    for line in lines {
        let (num, vec) = parse_line(line);
        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;

        let mut valid = true;

        for (r, g, b) in vec {
            if (r > 12) || (g > 13) || (b > 14) {
                valid = false;
            }
            min_r = if r > min_r {r} else {min_r};
            min_g = if g > min_g {g} else {min_g};
            min_b = if b > min_b {b} else {min_b};
        }

        if valid {
            output_part_1 += num;
        }

        output_part_2 += min_r * min_g * min_b;

    }

    println!("PART 1: {output_part_1}");
    println!("PART 2: {output_part_2}");
}
