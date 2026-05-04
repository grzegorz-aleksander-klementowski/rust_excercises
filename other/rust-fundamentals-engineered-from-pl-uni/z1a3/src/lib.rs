use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

/* Napisz program służący do konwersji wartości temperatury podanej w stopniach Celsjusza na stopnie w skali Fahrenheita

```
F = 32 + 9/5 C
``` */
#[derive(Clone, Copy)]
struct Celsius<T>(T)
where
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Display
        + Copy;

#[derive(Clone, Copy)]
struct Fahrenheit<T>(T)
where
    T: Add<Output = T> + Clone + Div<Output = T> + Display + Copy;

impl<T> Add for Celsius<T>
where
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Display
        + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> Mul for Celsius<T>
where
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Display
        + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T> std::ops::Div for Celsius<T>
where
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Display
        + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<T> std::ops::Mul for Fahrenheit<T>
where
    T: Add<Output = T> + Clone + Div<Output = T> + Mul<Output = T> + Display + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T> std::ops::Div for Fahrenheit<T>
where
    T: Add<Output = T> + Clone + Div<Output = T> + Mul<Output = T> + Display + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<T> std::ops::Sub for Fahrenheit<T>
where
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Display
        + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/* impl<T> From<Celsius<T>> for f32
where
    T: Add<Output = T> + Clone + Div<Output = T> + std::convert::Into<f32>,
{
    fn from(value: Celsius<T>) -> Self {
        value.0.into()
    }
} */

impl<T> Celsius<T>
where
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Display
        + Copy,
{
    fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Fahrenheit<T>
where
    T: Add<Output = T> + Clone + Div<Output = T> + Mul<Output = T> + Display + Copy,
{
    fn into_inner(self) -> T {
        self.0
    }
}

// Conversion from C to F (F = 32 + ((9/5) * C))
impl From<Celsius<f32>> for Fahrenheit<f32> {
    fn from(value: Celsius<f32>) -> Self {
        Self((Celsius(32.0) + (Celsius(9.0) / Celsius(5.0)) * value).into_inner())
    }
}

// conversion from F to C (C =  (5/9) * (F-32))
impl From<Fahrenheit<f32>> for Celsius<f32> {
    fn from(value: Fahrenheit<f32>) -> Self {
        Self((Fahrenheit(5.0) / Fahrenheit(9.0) * (value - Fahrenheit(32.0))).into_inner())
    }
}

// Defining Display for Celsius
impl<T> Display for Celsius<T>
where
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Display
        + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.0;
        write!(f, "{c} C°")
    }
}

// Defining Display for Fahrenheit
impl<T> Display for Fahrenheit<T>
where
    T: Add<Output = T>
        + Clone
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Display
        + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.0;
        write!(f, "{c} F°")
    }
}
