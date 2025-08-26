use std::{fs::File, io::Read};

pub(crate) fn advent13() {
    let mut contents = String::new();
    File::open("inputs/advent13.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let mut machines = vec![];
    let parse_fns = [
        |st: &str| {
            st.replace("Button A: X+", "")
                .split(", Y+")
                .map(str::trim)
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect::<Vec<i64>>()
        },
        |st: &str| {
            st.replace("Button B: X+", "")
                .split(", Y+")
                .map(str::trim)
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect::<Vec<i64>>()
        },
        |st: &str| {
            st.replace("Prize: X=", "")
                .split(", Y=")
                .map(str::trim)
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect::<Vec<i64>>()
        },
    ];
    let mut machine = Machine {
        a: Coords { x: 0, y: 0 },
        b: Coords { x: 0, y: 0 },
        prize: Coords { x: 0, y: 0 },
    };
    contents.split("\n\n").for_each(|str| {
        str.split("\n").enumerate().for_each(|(i, s)| {
            if s == "" {
                return;
            }
            let co = parse_fns[i % 3](s);
            match i % 3 {
                0 => {
                    machine.a = Coords { x: co[0], y: co[1] };
                }
                1 => {
                    machine.b = Coords { x: co[0], y: co[1] };
                }
                2 => {
                    machine.prize = Coords { x: co[0], y: co[1] };
                    machines.push(machine.clone())
                }
                _ => {}
            };
        })
    });
    println!("{:?}", machines[machines.len() - 1]);
    let mut sum = 0;
    // println!("{machines:?}");
    machines.iter().for_each(|machine| {
        let mut min = 0;
        (0..101).for_each(|a| {
            (0..101).for_each(|b| {
                if (a * machine.a.x) + (b * machine.b.x) == machine.prize.x
                    && (a * machine.a.y) + (b * machine.b.y) == machine.prize.y
                {
                    if 3 * a + b < min || min == 0 {
                        min = 3 * a + b
                    }
                }
            });
        });
        sum += min;
    });
    println!("{sum}");
    sum = 0;
    machines.iter().for_each(|machine| {
        let mut machine = machine.clone();
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;

        let x0 = machine.a.x;
        let y0 = machine.a.y;
        let x1 = machine.b.x;
        let y1 = machine.b.y;
        let x2 = machine.prize.x;
        let y2 = machine.prize.y;

        let b = (y2 - (y0 * x2 / x0)) / (y1 - (x1 * y0 / x0));
        let _b = ((y2 * x0) - (y0 * x2)) / ((y1 * x0) - (x1 * y0));
        let a = (x2 - (b * x1)) / x0;
        let _a = (x2 / x0) - (b * x1 / x0);
        let a_source = ((y2 * x1) - (y1 * x2)) / ((y0 * x1) - (x0 * y1));
        let b_source = (y2 - (a_source * y0)) / y1;

        let ax0bx1 = (a * x0) + (_b * x1);
        let ax0_bx1 = (a * x0) + (b * x1);
        let aax0bx1 = (_a * x0) + (b * x1);
        let aax0bbx1 = (_a * x0) + (_b * x1);
        let asx0bsx0 = (a_source * x0) + (b_source * x1);
        let ay0by1 = (a * y0) + (_b * y1);
        let ay0bby1 = (a * y0) + (b * y1);
        let aay0by1 = (_a * y0) + (b * y1);
        let aay0bby1 = (_a * y0) + (_b * y1);
        let asy0bsy0 = (a_source * y0) + (b_source * y1);
        // fmt.Println(ax0bx1, x2)

        if a >= 0 && _b >= 0 && ax0bx1 == x2 && ay0by1 == y2 {
            sum += (a * 3) + _b
        } else if a >= 0 && b >= 0 && ax0_bx1 == x2 && ay0bby1 == y2 {
            sum += (a * 3) + b
        } else if _a >= 0 && b >= 0 && aax0bx1 == x2 && aay0by1 == y2 {
            sum += (_a * 3) + b
        } else if _a >= 0 && _b >= 0 && _a < 100 && _b < 100 && aax0bbx1 == x2 && aay0bby1 == y2 {
            sum += (_a * 3) + _b
        } else if a_source >= 0 && b_source >= 0 && asx0bsx0 == x2 && asy0bsy0 == y2 {
            sum += (a_source * 3) + b_source
        }
    });
    println!("{sum}");
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Coords {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug)]
struct Machine {
    a: Coords,
    b: Coords,
    prize: Coords,
}
