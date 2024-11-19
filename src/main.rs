use std::fmt::{self, Binary, Debug, Display};

pub struct BitDate(u32);
// y << 10 | m << 6 | d << 1 | l

impl Display for BitDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year(), self.month(), self.day())
    }
}

impl Debug for BitDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year(), self.month(), self.day())
    }
}

impl Binary for BitDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.ymdl())
    }
}

// Date inc and dec should be performed using bit manipulations. Don't upack values.

impl BitDate {
    pub const MAX: u32 = 999_999 << 10 | 12 << 6 | 31 << 1;
    pub const MIN: u32 = 1_000 << 10 | 1 << 6 | 1 << 1;

    const YEAR_MASK: u32 = 0x003FFFFF;
    const MONTH_MASK: u32 = 0xF;
    const DAY_MASK: u32 = 0x1F;
    const LEAP_YEAR_MASK: u32 = 0x01;

    const DAYS_IN_YEAR: u64 = 31 << 55
        | 30 << 50
        | 31 << 45
        | 30 << 40
        | 31 << 35
        | 31 << 30
        | 30 << 25
        | 31 << 20
        | 30 << 15
        | 31 << 10
        | 28 << 5
        | 31;
    const DAYS_IN_LEAP_YEAR: u64 = 31 << 55
        | 30 << 50
        | 31 << 45
        | 30 << 40
        | 31 << 35
        | 31 << 30
        | 30 << 25
        | 31 << 20
        | 30 << 15
        | 31 << 10
        | 29 << 5
        | 31;

    pub fn from_ymd(y: u32, m: u32, d: u32) -> Option<Self> {
        let leap_year = Self::leap_year_bit(y);
        if (1..=12).contains(&m) && d <= Self::days_in_month(m, leap_year) {
            let packed_year = Self::pack_year(y);
            let packed_month = Self::pack_month(m);
            let packed_day = Self::pack_day(d);

            let packed_leap_year = Self::pack_leap_year(leap_year);
            Some(Self::from_packed_integer(
                packed_year | packed_month | packed_day | packed_leap_year,
            ))
        } else {
            None
        }
    }

    fn from_packed_integer(ymdl: u32) -> Self {
        Self(ymdl)
    }

    fn ymdl(&self) -> u32 {
        self.0
    }

    fn days_in_month(month: u32, leap_year_bit: u32) -> u32 {
        let source = if leap_year_bit == 1 {
            Self::DAYS_IN_LEAP_YEAR
        } else {
            Self::DAYS_IN_YEAR
        };
        ((source >> ((month - 1) * 5)) & 0x1F) as u32
    }

    fn pack_year(year: u32) -> u32 {
        year << 10
    }

    fn pack_month(month: u32) -> u32 {
        month << 6
    }

    fn pack_day(day: u32) -> u32 {
        day << 1
    }

    fn pack_leap_year(leap_year: u32) -> u32 {
        leap_year
    }

    fn unpack_year(ymdl: u32) -> u32 {
        ymdl >> 10 & Self::YEAR_MASK
    }

    fn unpack_month(ymdl: u32) -> u32 {
        ymdl >> 6 & Self::MONTH_MASK
    }

    fn unpack_day(ymdl: u32) -> u32 {
        ymdl >> 1 & Self::DAY_MASK
    }

    fn unpack_leap_year(ymdl: u32) -> u32 {
        ymdl & Self::LEAP_YEAR_MASK
    }

    fn leap_year_bit(year: u32) -> u32 {
        if year & 3 != 0 {
            return 0b0;
        }

        if year % 100 == 0 && year % 400 != 0 {
            return 0b0;
        }
        0b1
    }

    pub fn year(&self) -> u32 {
        Self::unpack_year(self.ymdl())
    }

    pub fn month(&self) -> u32 {
        Self::unpack_month(self.ymdl())
    }

    pub fn day(&self) -> u32 {
        Self::unpack_day(self.ymdl())
    }

    pub fn leap_year(&self) -> u32 {
        Self::unpack_leap_year(self.ymdl())
    }

    //pub fn eom(&mut self) -> &mut self {}

    //pub update_year()

    //pub update_month()

    //pub update_day()

    //pub add_days()

    // Sets day value to eom if current date value is not
    // allowed in the new month.
    //pub add_months()

    //pub sub_days()

    //pub sub_months()
}

fn main() {
    if let Some(date) = BitDate::from_ymd(2024, 2, 29) {
        println!("{:?}", &date.year());
        println!("{:?}", &date.month());
        println!("{:?}", &date.day());
    } else {
        println!("Invalid date. HELP!")
    }
}
