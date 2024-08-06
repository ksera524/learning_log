use rand::Rng;

struct MeetingTime {
    start_hour: i32,
    end_hour: i32,
}

fn create_meeting_api_call(names: Vec<String>, meeting_time: MeetingTime) {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();

    if random_number < 0.25 {
        panic!("Panic! You hit the 25% chance.");
    } else {
        println!("side effect");
    }
}

fn calender_entires_api_call(name: String) -> Vec<MeetingTime> {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();

    if random_number < 0.25 {
        panic!("Panic! You hit the 25% chance.");
    }

    if name == *"Alice" {
        vec![
            MeetingTime {
                start_hour: 8,
                end_hour: 10,
            },
            MeetingTime {
                start_hour: 11,
                end_hour: 12,
            },
        ]
    } else if name == *"Bob" {
        vec![MeetingTime {
            start_hour: 9,
            end_hour: 10,
        }]
    } else {
        return vec![MeetingTime {
            start_hour: rng.gen_range(0..5) + 8,
            end_hour: rng.gen_range(0..4) + 13,
        }];
    }
}
