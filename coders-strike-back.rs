use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug, Copy, Clone)]
pub struct Pod {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    angle: i32,
    next_checkpoint_id: usize,
    thrust: i32,
}

impl Pod {
    fn new(
            x: i32,
            y: i32,
            vx: i32,
            vy: i32,
            angle: i32,
            next_checkpoint_id: usize,
            thrust: i32,
        ) -> Pod {
        Pod {
            x,
            y,
            vx,
            vy,
            angle,
            next_checkpoint_id,
            thrust,
        }
    }

    fn update(
        &mut self,
        x: i32,
        y: i32,
        vx: i32,
        vy: i32,
        angle: i32,
        next_checkpoint_id: usize
    ) {
        self.x = x;
        self.y = y;
        self.vx = vx;
        self.vy = vy;
        self.angle = angle;
        self.next_checkpoint_id = next_checkpoint_id;
    }

    fn distance (&mut self, checkpoint: CheckPoint) -> i32 {
        (
            ((self.x - checkpoint.x) as f64).powi(2)
            + ((self.y - checkpoint.y) as f64).powi(2)
        ).sqrt() as i32
    }


    //Вычисляем скорость
    fn thrust (&mut self) {

    }
}

#[derive(Debug, Copy, Clone)]
pub struct CheckPoint {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Track {
    checkpoints: Vec<CheckPoint>,
    laps: i32,
    checkpoint_count: usize,
}

impl Track {
    fn new(checkpoints: Vec<CheckPoint>, laps: i32, checkpoint_count: usize) -> Track {
        Track {
            checkpoints: checkpoints,
            laps: laps,
            checkpoint_count: checkpoint_count
        }
    }

    fn equal(point1: CheckPoint, point2: CheckPoint) -> bool {
        if (point1.x == point2.x && point1.y == point2.y) {
            return true;
        }
        return false;
    }

    fn pointById(&mut self, checkpoint_id: usize) -> CheckPoint {
        self.checkpoints[checkpoint_id]
    }

    fn lapByPoint(&mut self, checkpoint: CheckPoint) {
        let point_position = self.checkpoints.into_iter().position(
                |x: CheckPoint| Track::equal(x, checkpoint)
            );

            match point_position {
                None => eprintln!("Error"),
                Some(position) => {
                    eprintln!("{0}", position);
                }
            }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let laps = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let checkpoint_count = parse_input!(input_line, usize);

    let mut checkpoints: Vec<CheckPoint> = Vec::with_capacity(checkpoint_count);
    let mut my_pods: Vec<Pod> = Vec::with_capacity(2);
    let mut enemy_pods: Vec<Pod> = Vec::with_capacity(2);

    for i in 0..checkpoint_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let checkpoint_x = parse_input!(inputs[0], i32);
        let checkpoint_y = parse_input!(inputs[1], i32);

        checkpoints.push(
            CheckPoint {
                x: checkpoint_x,
                y: checkpoint_y,
            }
        );
    }

    let mut TrackInfo: Track = Track::new(
        checkpoints,
        laps,
        checkpoint_count,
    );


    // game loop
    loop {
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            if (my_pods.len() > 1) {
                my_pods[i].update(
                    parse_input!(inputs[0], i32),
                    parse_input!(inputs[1], i32),
                    parse_input!(inputs[2], i32),
                    parse_input!(inputs[3], i32),
                    parse_input!(inputs[4], i32),
                    parse_input!(inputs[5], usize),
                );
            } else {
                let mut pod = Pod::new(
                    parse_input!(inputs[0], i32),
                    parse_input!(inputs[1], i32),
                    parse_input!(inputs[2], i32),
                    parse_input!(inputs[3], i32),
                    parse_input!(inputs[4], i32),
                    parse_input!(inputs[5], usize),
                    100,
                );
                my_pods.push(pod);
            }
        }
        
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            if (enemy_pods.len() > 1) {
                enemy_pods[i].update(
                    parse_input!(inputs[0], i32),
                    parse_input!(inputs[1], i32),
                    parse_input!(inputs[2], i32),
                    parse_input!(inputs[3], i32),
                    parse_input!(inputs[4], i32),
                    parse_input!(inputs[5], usize),
                );
            } else {
                let mut pod = Pod::new(
                    parse_input!(inputs[0], i32),
                    parse_input!(inputs[1], i32),
                    parse_input!(inputs[2], i32),
                    parse_input!(inputs[3], i32),
                    parse_input!(inputs[4], i32),
                    parse_input!(inputs[5], usize),
                    100,
                );
                enemy_pods.push(pod);
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        for i in 0..my_pods.len() {
            let mut pod: Pod = my_pods[i];
            let next: CheckPoint = TrackInfo.pointById(pod.next_checkpoint_id);
            let mut str_boost: &str = "BOOST";

            TrackInfo.lapByPoint(next);

            eprintln!(
                "My: x: {0}, y: {1}, boost: {2}",
                pod.vx, pod.vy, str_boost
            );
            println!("{0} {1} {2}", next.x, next.y, str_boost);
        }

        for i in 0..enemy_pods.len() {
            let pod: Pod = enemy_pods[i];
            // eprintln!(
            //     "Enemy: x: {0}, y: {1}, next_id: {2}",
            //     pod.x, pod.y, pod.next_checkpoint_id
            // );
        }

        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"
    }
}

