#[derive(Debug, Copy, Clone)]
enum Day {
    None,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive
}

impl From<i32> for Day  {
    fn from(num: i32) -> Self {
        match num {
            1 => Day::One,
            2 => Day::Two,
            3 => Day::Three,
            4 => Day::Four,
            5 => Day::Five,
            6 => Day::Six,
            7 => Day::Seven,
            8 => Day::Eight,
            9 => Day::Nine,
            10 => Day::Ten,
            11 => Day::Eleven,
            12 => Day::Twelve,
            13 => Day::Thirteen,
            14 => Day::Fourteen,
            15 => Day::Fifteen,
            16 => Day::Sixteen,
            17 => Day::Seventeen,
            18 => Day::Eighteen,
            19 => Day::Nineteen,
            20 => Day::Twenty,
            21 => Day::TwentyOne,
            22 => Day::TwentyTwo,
            23 => Day::TwentyThree,
            24 => Day::TwentyFour,
            25 => Day::TwentyFive,
            _ => Day::None
        }
    }
}

pub fn run_day(day: Day) -> Result<(), std::Error> {
    todo!();
    return Ok(());
}