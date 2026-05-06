// 5. Napisz program, który dla danych dwóch poprawnych pór jednej doby (w postaci całkowitych godzin, minut i sekund) wyświetla różnicę czasów (także w postaci analogicznej trójki, z minutami i sekundami w przedziale [0;59]).

struct Godzina(isize);
struct Minuta(isize);
struct Sekunda(isize);
struct Czas(Godzina, Minuta, Sekunda);

impl From<Godzina> for Sekunda {
    fn from(value: Godzina) -> Self {
        Sekunda(value.0 * 3600)
    }
}

impl From<Sekunda> for Godzina {
    fn from(value: Sekunda) -> Self {
        Godzina(value.0 / 3600)
    }
}

impl From<Minuta> for Sekunda {
    fn from(value: Minuta) -> Self {
        Sekunda(value.0 * 60)
    }
}

impl From<Sekunda> for Minuta {
    fn from(value: Sekunda) -> Self {
        Minuta(value.0 / 60)
    }
}

impl std::ops::Sub for Czas {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod test {
    fn test_różnica_czasu() {
        todo!()
    }
}
