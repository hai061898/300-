// Input: date = "20th Oct 2052"

// Output: "2052-10-20"

use regex::Regex;

fn reformat_date(date: String) -> String {
    let re = Regex::new(r"(\d{1,2}) ([A-Za-z]+) (\d{4})").unwrap(); #note
    let caps = re.captures(&date).unwrap();
    let month = match caps[2].as_ref() {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => panic!("Invalid month"),
    };
    let day = caps[1].parse::<u32>().unwrap();
    let year = caps[3].to_string();
    format!("{}-{}-{:02}", year, month, day)
}

fn main() {
    let date1 = String::from("20th Oct 2052");
    let date2 = String::from("6th Jun 1933");
    let date3 = String::from("4th Mar 1900");

    println!("{}", reformat_date(date1));
    println!("{}", reformat_date(date2));
    println!("{}", reformat_date(date3));
}


