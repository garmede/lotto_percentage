use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    const SIMUL_COUNT: i32 = 10000000;
    // 843회차 당첨 번호
    let lotto_num = vec![19, 32, 37, 40, 41, 43];
    let lotto_bonus = 45;

    let sys_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards");

    let mut medal = (0, 0, 0, 0, 0, 0);
    for _ in 1..SIMUL_COUNT + 1 {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<i32> = (1..46).collect();
        nums.shuffle(&mut rng);

        let slice_nums = &nums[1..7];

        let mut match_num = 0;
        for num in slice_nums {
            for l_num in &lotto_num {
                if num == l_num {
                    match_num += 1;
                    continue;
                }
            }
        }

        match match_num {
            0 => {
                medal.0 += 1;
            }
            1 => {
                medal.0 += 1;
            }
            2 => {
                medal.0 += 1;
            }
            3 => {
                medal.1 += 1;
            }
            4 => {
                medal.2 += 1;
            }
            5 => {
                let mut bonus = false;
                for num in slice_nums {
                    if num == &lotto_bonus {
                        bonus = true;
                        continue;
                    }
                }
                if bonus {
                    medal.4 += 1;
                } else {
                    medal.3 += 1;
                }
            }
            6 => {
                medal.5 += 1;
            }
            _ => println!("에러!"),
        }
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards");
    let difference = now.as_secs_f64() - sys_time.as_secs_f64();
    println!(
        "Time:{}, Count:{}, 1등:{}, 2등:{}, 3등:{}, 4등:{}, 5등:{}, 꽝:{}",
        difference, SIMUL_COUNT, medal.5, medal.4, medal.3, medal.2, medal.1, medal.0
    );
    println!(
        "확률 - 1등: {}%, 2등: {}%, 3등: {}%, 4등:{}%, 5등:{}%, 꽝:{}%",
        medal.5 as f64 / SIMUL_COUNT as f64 * 100.0 as f64,
        medal.4 as f64 / SIMUL_COUNT as f64 * 100.0 as f64,
        medal.3 as f64 / SIMUL_COUNT as f64 * 100.0 as f64,
        medal.2 as f64 / SIMUL_COUNT as f64 * 100.0 as f64,
        medal.1 as f64 / SIMUL_COUNT as f64 * 100.0 as f64,
        medal.0 as f64 / SIMUL_COUNT as f64 * 100.0 as f64,
    )
}
