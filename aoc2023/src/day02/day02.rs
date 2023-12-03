use aoc2023::read_lines;

fn main() {
    let path = "src/day02/input.txt";
    let lines = read_lines(path);
    solve_part_1(lines);
}

fn solve_part_1(lines: Vec<String>) -> u128 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut result = 0;
    for line in lines {
        //println!("{line}");
        let mut impossible_game = false;
        let game_split: Vec<&str> = line.split(":").collect();
        let game_no = game_split[0];
        let game_no_str: Vec<&str> = game_no.split(" ").collect();
        let game_no_int: u128 = game_no_str[1].trim().parse().unwrap();
        let game_hands: Vec<&str> = game_split[1].split(";").collect();
        for hand in game_hands {
            let hand_cubes: Vec<&str> = hand.split(",").collect();
            for h in hand_cubes {
                //println!("{h}");
                let count_color: Vec<&str> = h.trim().split(" ").collect();
                //println!("{:?}", count_color);
                let count: u128 = count_color[0].trim().parse().unwrap();
                let color = count_color[1];
                match color {
                    "red" => {
                        if count > max_red {
                            impossible_game = true;
                        }
                    }
                    "blue" => {
                        if count > max_blue {
                            impossible_game = true;
                        }
                    }
                    "green" => {
                        if count > max_green {
                            impossible_game = true;
                        }
                    }
                    _ => println!("oops"),
                }
            }
        }
        if !impossible_game {
            println!("Possible game: {line}");
            result += game_no_int;
        }
    }
    println!("{result}");
    return result;
}
