pub struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    pub fn from_boarding_pass_str(id: &str) -> Self {
        let mut row = 0;
        for (i, letter) in id.bytes().take(7).enumerate() {
            if letter == b'B' {
                row |= 1 << (6 - i);
            }
        }

        let mut column = 0;
        for (i, letter) in id.bytes().skip(7).enumerate() {
            if letter == b'R' {
                column |= 1 << (2 - i);
            }
        }

        Self { row, column }
    }

    pub fn id(&self) -> u16 {
        self.row as u16 * 8 + self.column as u16
    }
}

#[cfg(test)]
mod tests {
    use super::Seat;

    fn get_test_input() -> String {
        std::fs::read_to_string("input/day_05").unwrap()
    }

    #[test]
    fn single_seat() {
        let seat = Seat::from_boarding_pass_str("BFFFBBFRRR");
        assert_eq!(70, seat.row);
        assert_eq!(7, seat.column);
        assert_eq!(567, seat.id());
    }

    #[test]
    fn part1() {
        let max = get_test_input()
            .lines()
            .map(|line| Seat::from_boarding_pass_str(line).id())
            .max()
            .unwrap();

        assert_eq!(818, max);
    }

    #[test]
    fn part2() {
        let mut seat_ids = get_test_input()
            .lines()
            .map(|line| Seat::from_boarding_pass_str(line).id())
            .collect::<Vec<u16>>();

        seat_ids.sort_unstable();
        let min_seat_id = seat_ids.first().unwrap();

        let mut missing_seat = None;
        for (i, seat_id) in seat_ids.iter().copied().enumerate() {
            // The first time we find a gap, we just have just passed the missing
            // seat.
            if seat_id != min_seat_id + i as u16 {
                missing_seat = Some(seat_id - 1);
                break;
            }
        }

        assert_eq!(559, missing_seat.unwrap());
    }
}
