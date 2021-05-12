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
    pub next_checkpoint_id: usize,
    pub laps: i32,
}

fn build_pod (x: i32, y: i32, vx: i32, vy: i32, angle: i32, next_checkpoint_id: usize, laps: i32) -> Pod {
    Pod {
        x,
        y,
        vx,
        vy,
        angle,
        next_checkpoint_id,
        laps,
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

    let TrackInfo: Track = Track{
        checkpoints,
        laps,
        checkpoint_count,
    };

    eprintln!("{:?}", TrackInfo.checkpoints);

    // game loop
    loop {
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
        
            let mut pod = build_pod(
                parse_input!(inputs[0], i32),
                parse_input!(inputs[1], i32),
                parse_input!(inputs[2], i32),
                parse_input!(inputs[3], i32),
                parse_input!(inputs[4], i32),
                parse_input!(inputs[5], usize),
                0,
            );


            eprintln!("Next: {0}", parse_input!(inputs[5], i32));

            if (pod.next_checkpoint_id == 0) {
                pod.laps = my_pods[i].laps as i32 + 1;
            }

            if (my_pods.len() > 1) {
                my_pods[i] = pod;
            } else {
                my_pods.push(pod);
            }
        }
        
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let mut pod = build_pod(
                parse_input!(inputs[0], i32),
                parse_input!(inputs[1], i32),
                parse_input!(inputs[2], i32),
                parse_input!(inputs[3], i32),
                parse_input!(inputs[4], i32),
                parse_input!(inputs[5], usize),
                1,
            );
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        for i in 0..my_pods.len() {
            let pod: Pod = my_pods[i];
            let next: CheckPoint = TrackInfo.checkpoints[pod.next_checkpoint_id];
            let mut boost: i32 = 100;
            if ((pod.x < next.x && pod.x + pod.vx + 400 >= next.x) || 
                (pod.x > next.x && pod.x - pod.vx - 400 <= next.x)
            ) {
                boost = 0;
            }
            
            eprintln!(
                "x: {0}, y: {1}, next_id: {2}, laps: {3}",
                pod.x, pod.y, pod.next_checkpoint_id, pod.laps
            );
            println!("{0} {1} {2}", next.x, next.y, boost);
        }

        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"
    }
}

