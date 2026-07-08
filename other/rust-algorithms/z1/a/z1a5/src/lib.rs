// 5. Napisz program, który dla danych dwóch poprawnych pór jednej doby (w postaci całkowitych godzin, minut i sekund) wyświetla różnicę czasów (także w postaci analogicznej trójki, z minutami i sekundami w przedziale [0;59]).
// EN: 5. Write a program that, for two valid times of one day (in the form of total hours, minutes and seconds), displays the time difference (also in the form of an analogous triplet, with minutes and seconds in the interval [0;59]).
// ASSUME TIME–1 ALWAYS >=TIME–2
#[derive(std::fmt::Debug, PartialEq, Eq)]
pub struct Hour(pub isize);
#[derive(std::fmt::Debug, PartialEq, Eq)]
pub struct Minute(pub isize);
#[derive(std::fmt::Debug, PartialEq, Eq)]
pub struct Second(pub isize);
#[derive(std::fmt::Debug, PartialEq, Eq)]
pub struct Time(pub Hour, pub Minute, pub Second);

impl From<Hour> for Second {
    fn from(value: Hour) -> Self {
        Self(value.0 * 3600)
    }
}

impl From<Minute> for Second {
    fn from(value: Minute) -> Self {
        Self(value.0 * 60)
    }
}

impl From<Second> for Time {
    fn from(value: Second) -> Self {
        let sec = value.0;
        let h = Hour(sec / 3600);
        let sec = sec % 3600;
        let min = Minute(sec / 60);
        let sec = Second(sec % 60);

        Self(h, min, sec)
    }
}

impl From<Time> for Second {
    fn from(time: Time) -> Self {
        let hours_to_sec: Self = time.0.into();
        let min_to_sec: Self = time.1.into();
        let sec: Self = time.2;
        Self(hours_to_sec.0 + min_to_sec.0 + sec.0)
    }
}

impl std::ops::Sub for Time {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let time1 = self;
        let time2 = rhs;

        let sec1: Second = time1.into();
        let sec2: Second = time2.into();

        let time_result_in_sec = Second(sec1.0 - sec2.0);
        let output: Self = time_result_in_sec.into();
        output
    }
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
        let time2 = Time(Hour(23), Minute(59), Second(59));

        let result = time1 - time2;

        assert_eq!(result, Time(Hour(0), Minute(0), Second(0)));
    }
}
