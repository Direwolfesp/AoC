#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Year(u64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Height {
    /// Centimeters
    Cm(u64),
    /// Inches
    In(u64),
    /// Unspecified
    Unknown(u64),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color<'a>(&'a str);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ID<'a>(&'a str);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Passport<'a> {
    birth_year: Year,
    issue_year: Year,
    expiration_year: Year,
    height: Height,
    hair_color: Color<'a>,
    eye_color: Color<'a>,
    passport_id: ID<'a>,
    country_id: Option<ID<'a>>,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct PassportBuilder<'a> {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Height>,
    hair_color: Option<Color<'a>>,
    eye_color: Option<Color<'a>>,
    passport_id: Option<ID<'a>>,
    country_id: Option<ID<'a>>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("Unknown field: {0}")]
    UnknownField(String),

    #[error("could not parse {0}: {1}")]
    ParseError(&'static str, &'static str),
}

impl<'a> PassportBuilder<'a> {
    fn parse(input: &'a str) -> Result<Self, Error> {
        let mut b = PassportBuilder::default();

        for part in input.split_whitespace() {
            let mut parts = part.split(":").take(2);
            let field = parts.next().expect("field is present");
            let value = parts.next().expect("value is present");
            match field {
                "byr" => {
                    if value.len() != 4 {
                        return Err(Error::ParseError("birth_year", "Must have 4 digits"));
                    }
                    let year: u64 = value
                        .parse()
                        .or(Err(Error::ParseError("birth_year", "Not a number")))?;

                    if !(1920..=2002).contains(&year) {
                        return Err(Error::ParseError("birth_year", "Invalid range"));
                    }

                    b.birth_year = Some(Year(year));
                }
                "iyr" => {
                    if value.len() != 4 {
                        return Err(Error::ParseError("issue_year", "Must have 4 digits"));
                    }
                    let year: u64 = value
                        .parse()
                        .or(Err(Error::ParseError("issue_year", "Not a number")))?;

                    if !(2010..=2020).contains(&year) {
                        return Err(Error::ParseError("issue_year", "Invalid range"));
                    }
                    b.issue_year = Some(Year(value.parse().expect("Invalid year")));
                }
                "eyr" => {
                    if value.len() != 4 {
                        return Err(Error::ParseError("expiration_year", "Must have 4 digits"));
                    }
                    let year: u64 = value
                        .parse()
                        .or(Err(Error::ParseError("expiration_year", "Not a number")))?;

                    if !(2020..=2030).contains(&year) {
                        return Err(Error::ParseError("expiration_year", "Invalid range"));
                    }
                    b.expiration_year = Some(Year(value.parse().expect("Invalid year")));
                }
                "hgt" => {
                    b.height = match (value.strip_suffix("cm"), value.strip_suffix("in")) {
                        (Some(cm), None) => {
                            let height: u64 = cm
                                .parse()
                                .or(Err(Error::ParseError("height", "Not a number")))?;

                            if !(150..=193).contains(&height) {
                                return Err(Error::ParseError("height", "Invalid range"));
                            }

                            Some(Height::Cm(height))
                        }
                        (None, Some(inch)) => {
                            let height: u64 = inch
                                .parse()
                                .or(Err(Error::ParseError("height", "Not a number")))?;

                            if !(59..=76).contains(&height) {
                                return Err(Error::ParseError("height", "Invalid range"));
                            }

                            Some(Height::In(height))
                        }
                        (_, _) => None,
                    };
                }
                "hcl" => {
                    b.hair_color = match value.strip_prefix("#") {
                        Some(color) if color.len() == 6 => {
                            if i64::from_str_radix(color, 16).is_ok() {
                                Some(Color(value))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                }
                "ecl" => {
                    let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    b.eye_color = if valid_colors.contains(&value) {
                        Some(Color(value))
                    } else {
                        None
                    };
                }
                "pid" => {
                    b.passport_id = if value.len() == 9 {
                        if value.parse::<i64>().is_ok() {
                            Some(ID(value))
                        } else {
                            return Err(Error::ParseError("passport_id", "Not a number"));
                        }
                    } else {
                        None
                    };
                }
                "cid" => {
                    b.country_id = if value.parse::<i64>().is_ok() {
                        Some(ID(value))
                    } else {
                        return Err(Error::ParseError("country_id", "Not a number"));
                    };
                }
                unknown => return Err(Error::UnknownField(unknown.into())),
            }
        }

        Ok(b)
    }

    fn build(self) -> Result<Passport<'a>, Error> {
        Ok(Passport {
            birth_year: self.birth_year.ok_or(Error::MissingField("birth_year"))?,
            issue_year: self.issue_year.ok_or(Error::MissingField("issue_year"))?,
            expiration_year: self
                .expiration_year
                .ok_or(Error::MissingField("expiration_year"))?,
            height: self.height.ok_or(Error::MissingField("height"))?,
            hair_color: self.hair_color.ok_or(Error::MissingField("hair_color"))?,
            eye_color: self.eye_color.ok_or(Error::MissingField("eye_color"))?,
            passport_id: self.passport_id.ok_or(Error::MissingField("passport_id"))?,
            country_id: self.country_id,
        })
    }
}

pub fn main() {
    let input = include_str!("../input.txt");

    let sol: usize = input
        .split("\n\n")
        .map(|input| PassportBuilder::parse(input).and_then(|b| b.build()))
        .filter(Result::is_ok)
        .count();

    println!("Solution: {sol}");
}
