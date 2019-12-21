use aoc_runner_derive::aoc;
use itertools::Itertools;

struct Moon {
    pos: [i64; 3],
    vel: [i64; 3]
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            pos: [x, y, z],
            vel: [0, 0, 0]
        }
    }
}

#[aoc(day12, part1)]
fn total_energy(_input: &str) -> i64 {
    let mut moons = [
        Moon::new(-15, 1, 4),
        Moon::new(1, -10, -8),
        Moon::new(-5, 4, 9),
        Moon::new(4, 6, -2),
    ];

    for _ in 0..1000 {
        for (i, j) in (0..moons.len()).tuple_combinations() {
            for p in 0..3 {
                if moons[i].pos[p] < moons[j].pos[p] {
                    moons[i].vel[p] += 1;
                    moons[j].vel[p] -= 1;
                }

                if moons[i].pos[p] > moons[j].pos[p] {
                    moons[i].vel[p] -= 1;
                    moons[j].vel[p] += 1;
                }
            }
        }

        for moon in &mut moons {
            for i in 0..3 {
                let vel = moon.vel[i];
                moon.pos[i] += vel;
            }
        }
    }

    moons.iter()
        .map(|m| {
            let potential: i64 = m.pos.iter().cloned().map(i64::abs).sum();
            let kinetic: i64 = m.vel.iter().cloned().map(i64::abs).sum();

            potential * kinetic
        })
        .sum()
}
