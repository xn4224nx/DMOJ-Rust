/*
 * Waterloo 2026 Fall C - If I Could Turn Back Time
 * https://dmoj.ca/problem/waterloo2026fc
 */

const SECS_IN_HOUR: u32 = 3600;
const SECS_IN_MIN: u32 = 60;
const HOURS_IN_DAY: u32 = 24;

struct Clock {
    secs_from_midnight: u32,
}

impl Clock {
    /// Read a clock face in the HH:MM:SS format.
    fn new(raw_clock_face: &str) -> Self {
        let time_components = raw_clock_face
            .trim_end()
            .split(':')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        assert_eq!(time_components.len(), 3);

        return Clock {
            secs_from_midnight: SECS_IN_HOUR * time_components[0]
                + SECS_IN_MIN * time_components[1]
                + time_components[2],
        };
    }

    /// Move back to the previous second on the clock.
    fn prev_sec(&mut self) {
        self.secs_from_midnight = if self.secs_from_midnight == 0 {
            HOURS_IN_DAY * SECS_IN_HOUR - 1
        } else {
            self.secs_from_midnight.saturating_sub(1)
        };
    }

    /// Determine how many segments are illuminated on the clock.
    fn iluminated_segments(&self) -> u8 {
        let mut ilumn_segments = 0;
        let num_hour = self.secs_from_midnight / SECS_IN_HOUR;
        let num_mins = (self.secs_from_midnight % SECS_IN_HOUR) / SECS_IN_MIN;
        let num_secs = self.secs_from_midnight % SECS_IN_MIN;

        /* Extract the digits from the time. */
        let time_components = vec![
            num_hour / 10,
            num_hour % 10,
            num_mins / 10,
            num_mins % 10,
            num_secs / 10,
            num_secs % 10,
        ];

        /* What illumination does each digit have? */
        for comp_idx in 0..time_components.len() {
            ilumn_segments += match time_components[comp_idx] {
                1 => 2,
                2 | 3 | 5 => 5,
                4 => 4,
                0 | 6 | 9 => 6,
                7 => 3,
                8 => 7,
                _ => panic!("Only digits 0-9 are supported!"),
            };
        }
        return ilumn_segments;
    }

    /// Convert the seconds from midnight to HH:MM:SS
    fn current_face(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}",
            self.secs_from_midnight / SECS_IN_HOUR,
            (self.secs_from_midnight % SECS_IN_HOUR) / SECS_IN_MIN,
            self.secs_from_midnight % SECS_IN_MIN
        )
    }

    /// What is the last time on the clock that it had the current illumination?
    fn previous_time_with_same_illumination(&mut self) -> String {
        let current_ilumn = self.iluminated_segments();

        /* Go back and find the next time that has the same illumination. */
        loop {
            self.prev_sec();

            if current_ilumn == self.iluminated_segments() {
                break;
            }
        }
        return self.current_face();
    }
}

fn main() {
    let mut buffer = String::new();

    /* Read the initial time from STDIN. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    println!(
        "{}",
        Clock::new(&buffer).previous_time_with_same_illumination()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ilum_segments_from_total_sec_0() {
        let test_clock = Clock {
            secs_from_midnight: 0,
        };
        assert_eq!(test_clock.iluminated_segments(), 36);
    }

    #[test]
    fn ilum_segments_from_total_sec_1() {
        let test_clock = Clock {
            secs_from_midnight: 1,
        };
        assert_eq!(test_clock.iluminated_segments(), 32);
    }

    #[test]
    fn ilum_segments_from_total_sec_2() {
        let test_clock = Clock {
            secs_from_midnight: 86399,
        };
        assert_eq!(test_clock.iluminated_segments(), 32);
    }

    #[test]
    fn ilum_segments_from_total_sec_3() {
        let test_clock = Clock {
            secs_from_midnight: 29288,
        };
        assert_eq!(test_clock.iluminated_segments(), 39);
    }

    #[test]
    fn ilum_segments_from_total_sec_4() {
        let test_clock = Clock {
            secs_from_midnight: 84239,
        };
        assert_eq!(test_clock.iluminated_segments(), 31);
    }

    #[test]
    fn regurgitate_clock_0() {
        assert_eq!(
            Clock::new("00:00:00").current_face(),
            String::from("00:00:00")
        );
    }

    #[test]
    fn regurgitate_clock_1() {
        assert_eq!(
            Clock::new("12:34:56").current_face(),
            String::from("12:34:56")
        );
    }

    #[test]
    fn regurgitate_clock_2() {
        assert_eq!(
            Clock::new("23:59:59").current_face(),
            String::from("23:59:59")
        );
    }

    #[test]
    fn regurgitate_clock_3() {
        assert_eq!(
            Clock::new("12:59:00").current_face(),
            String::from("12:59:00")
        );
    }

    #[test]
    fn regurgitate_clock_4() {
        assert_eq!(
            Clock::new("10:10:10").current_face(),
            String::from("10:10:10")
        );
    }

    #[test]
    fn go_back_in_time_0() {
        let mut test_clock = Clock::new("00:00:00");
        test_clock.prev_sec();
        assert_eq!(test_clock.current_face(), String::from("23:59:59"));
    }

    #[test]
    fn go_back_in_time_1() {
        let mut test_clock = Clock::new("00:00:00");
        test_clock.prev_sec();
        test_clock.prev_sec();
        assert_eq!(test_clock.current_face(), String::from("23:59:58"));
    }

    #[test]
    fn go_back_in_time_2() {
        let mut test_clock = Clock::new("00:00:00");
        for _ in 0..(HOURS_IN_DAY * SECS_IN_HOUR) {
            test_clock.prev_sec();
        }
        assert_eq!(test_clock.current_face(), String::from("00:00:00"));
    }

    #[test]
    fn go_back_in_time_3() {
        let mut test_clock = Clock::new("10:00:01");
        test_clock.prev_sec();
        assert_eq!(test_clock.current_face(), String::from("10:00:00"));
    }

    #[test]
    fn go_back_in_time_4() {
        let mut test_clock = Clock::new("12:34:56");
        for _ in 0..(SECS_IN_HOUR * 2) {
            test_clock.prev_sec();
        }
        assert_eq!(test_clock.current_face(), String::from("10:34:56"));
    }

    #[test]
    fn go_back_in_time_5() {
        let mut test_clock = Clock::new("11:34:56");
        for _ in 0..SECS_IN_MIN {
            test_clock.prev_sec();
        }
        assert_eq!(test_clock.current_face(), String::from("11:33:56"));
    }
}
