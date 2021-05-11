use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug, Default, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Default::default()
    }
}

/**
 * Save the Planet.
 * Use less Fossil Fuel.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of points used to draw the surface of Mars.
    
    let mut surface = [Point::new(); 7];
    let mut is_even: bool = false;
    let gravity: f32 = -3.711;

    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
        surface[i] = Point{x: land_x, y: land_y };
    }


    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let hs = parse_input!(inputs[2], i32); // the horizontal speed (in m/s), can be negative.
        let vs = parse_input!(inputs[3], i32); // the vertical speed (in m/s), can be negative.
        let f = parse_input!(inputs[4], i32); // the quantity of remaining fuel in liters.
        let mut r = parse_input!(inputs[5], i32); // the rotation angle in degrees (-90 to 90).
        let mut p = parse_input!(inputs[6], i32); // the thrust power (0 to 4).


        let arcAngle = (-r as f32 *  std::f32::consts::PI) / 180.0;
        let xacc = arcAngle.sin() * p as f32;
        let yacc = arcAngle.cos() * p as f32 - gravity;
        let newHSpeed = hs as f32 + xacc;
        let newVSpeed = vs as f32 + yacc;
        let newX = (x as f32 + newHSpeed - xacc * 0.5) as i32;
        let newY = (y as f32 + newVSpeed - yacc * 0.5) as i32;

        let va_ground: usize = surface.iter()
            .position(|&distance| distance.x >= x + 500 && distance.x <= newX + 2000)
            .unwrap();
        

        if (surface.len() > va_ground) {
            let current_point = surface[va_ground - 1];
            let next_point = surface[va_ground];
            if (current_point.y == next_point.y) {
                is_even = true;
            }
        }

        if (is_even) {
            r = 10;
            if (y <= 250) {
                r = 0;
            }
            if (vs <= -40) {
                r = 10;
                p = 4;
            } else if (hs > 20) {
                r = 45;
                p = 4;
            }
        } else {
            r = -15;
            p = 3;
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        eprintln!("X: {0}; Y: {1}; Fuel: {2}; Rotation: {3}; Power: {4}; Is Even: {5}",
            x, y, f, r, p, is_even);

        // R P. R is the desired rotation angle. P is the desired thrust power.
        println!("{0} {1}", r, p);
    }
}

