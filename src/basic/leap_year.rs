// If the year is not divisible by 400 but is divisible by 100, it is not a leap year.
// If the year is not divisible by 100 but is divisible by 4, it is a leap year.
pub fn leap_year() {
    let year = 1900;
    if year % 400 == 0 || (year % 4 == 0 && year % !100 == 0) {
        print!("it is leap year");
    } else {
        print!("not a leap year");
    }
}
