// 5. Napisz program, który dla danych dwóch poprawnych pór jednej doby (w postaci całkowitych godzin, minut i sekund) wyświetla różnicę czasów (także w postaci analogicznej trójki, z minutami i sekundami w przedziale [0;59]).
// EN: 5. Write a program that, for two valid times of one day (in the form of total hours, minutes and seconds), displays the time difference (also in the form of an analogous triplet, with minutes and seconds in the interval [0;59]).
#[derive(std::fmt::Debug, PartialEq, Eq)]
struct Hour(isize);
#[derive(std::fmt::Debug, PartialEq, Eq)]
struct Minute(isize);
#[derive(std::fmt::Debug, PartialEq, Eq)]
struct Second(isize);
#[derive(std::fmt::Debug, PartialEq, Eq)]
struct Time(Hour, Minute, Second);

impl From<Hour> for Second {
    fn from(value: Hour) -> Self {
        Second(value.0 * 3600)
    }
}

impl From<Minute> for Second {
    fn from(value: Minute) -> Self {
        Second(value.0 * 60)
    }
}

impl From<Second> for Time {
    fn from(value: Second) -> Self {
        let sec = value.0;
        let h = Hour(sec / 3600);
        let sec = sec % 3600;
        let min = Minute(sec / 60);
        let sec = Second(sec % 60);

        Time(h, min, sec)
    }
}

impl std::ops::Sub for Time {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

enum TimeError {
    IncorrectTimeSubstract(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_difference_time() {
        let time1 = Time(Hour(2), Minute(25), Second(15));
        let time2 = Time(Hour(1), Minute(5), Second(45));

        let result = time1 - time2;

        assert_eq!(result, Time(Hour(1), Minute(19), Second(30)));
    }
    #[test]
    fn test_if_eq_zero() {
        let time1 = Time(Hour(1), Minute(0), Second(0));
        let time2 = Time(Hour(1), Minute(0), Second(0));

        let result = time1 - time2;

        assert_eq!(result, Time(Hour(0), Minute(0), Second(0)));
    }

    #[test]
    fn test_borrow_minutes_one() {
        let time1 = Time(Hour(1), Minute(10), Second(0));
        let time2 = Time(Hour(0), Minute(20), Second(0));

        let result = time1 - time2;

        assert_eq!(result, Time(Hour(0), Minute(50), Second(0)));
    }

    #[test]
    fn test_borrow_seconds() {
        let time1 = Time(Hour(2), Minute(10), Second(5));
        let time2 = Time(Hour(1), Minute(9), Second(50));

        let result = time1 - time2;

        assert_eq!(result, Time(Hour(1), Minute(0), Second(15)));
    }

    #[test]
    fn test_borrow_minutes_two() {
        let time1 = Time(Hour(2), Minute(0), Second(5));
        let time2 = Time(Hour(1), Minute(59), Second(50));

        let result = time1 - time2;

        assert_eq!(result, Time(Hour(0), Minute(0), Second(15)));
    }

    #[test]
    fn test_max_bounaries() {
        let time1 = Time(Hour(23), Minute(59), Second(59));
        let time2 = Time(Hour(0), Minute(59), Second(50));

        let result = time1 - time2;

        assert_eq!(result, Time(Hour(0), Minute(0), Second(15)));
    }

    #[test]
    fn test_next_day() {
        let time1 = Time(Hour(23), Minute(59), Second(59));
        let time2 = Time(Hour(0), Minute(59), Second(50));

        let result = time1 - time2;

        assert_eq!(result, Time(Hour(0), Minute(0), Second(15)));
    }
}
