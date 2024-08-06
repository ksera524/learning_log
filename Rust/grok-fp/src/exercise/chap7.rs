#[derive(PartialEq, Debug, Clone)]
struct Location(String);

impl From<String> for Location {
    fn from(s: String) -> Self {
        Location(s)
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Genre(String);

impl From<String> for Genre {
    fn from(s: String) -> Self {
        Genre(s)
    }
}

#[derive(PartialEq, Debug, Clone)]
struct YearsActiveStart(i32);

impl From<i32> for YearsActiveStart {
    fn from(s: i32) -> Self {
        YearsActiveStart(s)
    }
}

impl From<YearsActiveStart> for i32 {
    fn from(val: YearsActiveStart) -> Self {
        val.0
    }
}

#[derive(PartialEq, Debug, Clone)]
struct YearsActiveEnd(i32);

impl From<i32> for YearsActiveEnd {
    fn from(s: i32) -> Self {
        YearsActiveEnd(s)
    }
}

impl From<YearsActiveEnd> for i32 {
    fn from(val: YearsActiveEnd) -> Self {
        val.0
    }
}

#[derive(PartialEq, Debug, Clone)]
struct PeriodInYears {
    start: YearsActiveStart,
    end: Option<YearsActiveEnd>,
}

#[derive(PartialEq, Debug, Clone)]
enum YearsActive {
    StillActive {
        since: YearsActiveStart,
    },
    AcitiveBetween {
        start: YearsActiveStart,
        end: YearsActiveEnd,
    },
}

#[derive(PartialEq, Debug, Clone)]
struct Aritist {
    name: String,
    genre: MusicGunre,
    origin: Location,
    years_active: YearsActive,
}

impl Aritist {
    fn new(
        name: String,
        genre: MusicGunre,
        origin: Location,
        years_active: YearsActive,
    ) -> Aritist {
        Aritist {
            name,
            genre,
            origin,
            years_active,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum MusicGunre {
    HeavyMetal,
    Pop,
    HardRock,
}

fn was_artist_active(
    artist: &Aritist,
    year_start: &YearsActiveStart,
    year_end: &YearsActiveEnd,
) -> bool {
    match &artist.years_active {
        YearsActive::StillActive { since } => since.0 <= year_end.0,
        YearsActive::AcitiveBetween { start, end } => {
            start.0 <= year_end.0 && end.0 >= year_start.0
        }
    }
}

fn search_artists(
    artists: Vec<Aritist>,
    genres: Vec<MusicGunre>,
    locations: Vec<Location>,
    search_by_active_years: bool,
    active_after: YearsActiveStart,
    active_before: YearsActiveEnd,
) -> Vec<Aritist> {
    artists
        .into_iter()
        .filter(|artist| {
            (genres.is_empty() || genres.contains(&artist.genre))
                && (locations.is_empty() || locations.contains(&artist.origin))
                && (!search_by_active_years
                    || was_artist_active(artist, &active_after, &active_before))
        })
        .collect()
}

fn active_length(artist: &Aritist, current_year: i32) -> i32 {
    match &artist.years_active {
        YearsActive::StillActive { since } => current_year - since.0,
        YearsActive::AcitiveBetween { start, end } => end.0 - start.0,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn search_test() {
        let artist_one = Aritist::new(
            "Metallica".to_string(),
            MusicGunre::HeavyMetal,
            Location::from("U.S.".to_string()),
            YearsActive::StillActive {
                since: YearsActiveStart::from(1950),
            },
        );

        let artist_two = Aritist::new(
            "Led Zeppelin".to_string(),
            MusicGunre::HardRock,
            Location::from("England".to_string()),
            YearsActive::AcitiveBetween {
                start: YearsActiveStart::from(1968),
                end: YearsActiveEnd::from(1980),
            },
        );

        let artist_three = Aritist::new(
            "Bee Gees".to_string(),
            MusicGunre::Pop,
            Location::from("England".to_string()),
            YearsActive::AcitiveBetween {
                start: YearsActiveStart::from(1958),
                end: YearsActiveEnd::from(2003),
            },
        );

        let list: Vec<Aritist> = vec![artist_one.clone(), artist_two.clone(), artist_three.clone()];

        assert_eq!(
            vec![artist_three.clone()],
            search_artists(
                list,
                vec![MusicGunre::Pop],
                vec![Location::from("England".to_string())],
                true,
                YearsActiveStart::from(1950),
                YearsActiveEnd::from(2022)
            )
        );
    }
}
