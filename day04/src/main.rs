fn main() {
    part1();
    part2();
}

fn part1() {
    let range = 248345..746315;

    let mut count = 0;
    for password in range {
        let mut increasing = true;
        let mut double = false;
        let password = password.to_string();
        let mut chars = password.chars().peekable();
        while let Some(digit) = chars.next() {
            if let Some(next) = chars.peek() {
                if digit.to_digit(10).unwrap() > next.to_digit(10).unwrap() {
                    increasing = false;
                    break;
                }

                if digit.to_digit(10).unwrap() == next.to_digit(10).unwrap() {
                    double = true;
                }
            }
        }

        if increasing && double {
            count += 1;
        }
    }

    println!("{}", count);
}

fn part2() {
    let range = 248345..746315;

    let mut count = 0;
    for password in range {
        let mut increasing = true;
        let mut double = false;
        let mut found_double = false;
        let mut sequence = false;
        let password = password.to_string();
        let mut chars = password.chars().peekable();
        while let Some(digit) = chars.next() {
            if let Some(next) = chars.peek() {
                if digit.to_digit(10).unwrap() > next.to_digit(10).unwrap() {
                    increasing = false;
                    break;
                }

                if digit.to_digit(10).unwrap() == next.to_digit(10).unwrap() {
                    if sequence {
                        double = false;
                    } else {
                        double = true;
                        sequence = true;
                    }
                } else {
                    sequence = false;
                    if double {
                        found_double = true;
                    }
                }
            } else {
                if double {
                    found_double = true;
                }
            }
        }

        if increasing && found_double {
            count += 1;
        }
    }

    println!("{}", count);
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let password = 111233;
        let mut increasing = true;
        let mut double = false;
        let mut found_double = false;
        let mut sequence = false;
        let password = password.to_string();
        let mut chars = password.chars().peekable();
        while let Some(digit) = chars.next() {
            if let Some(next) = chars.peek() {
                if digit.to_digit(10).unwrap() > next.to_digit(10).unwrap() {
                    increasing = false;
                    break;
                }

                if digit.to_digit(10).unwrap() == next.to_digit(10).unwrap() {
                    if sequence {
                        double = false;
                    } else {
                        double = true;
                        sequence = true;
                    }
                } else {
                    sequence = false;
                    if double {
                        found_double = true;
                    }
                }
            } else {
                if double {
                    found_double = true;
                }
            }
        }

        if increasing && found_double {
            println!("!")
        }
    }
}
