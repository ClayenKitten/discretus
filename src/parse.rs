pub struct Identifier {
    ch: char,
    index: Option<u16>
}

impl Identifier {
    pub fn to_beautiful(&self) -> String {
        let index = match self.index {
            Some(index) => to_subscript(index),
            None => String::new(),
        };
        format!("{}{index}", self.ch)
    }
}

fn to_subscript(mut num: u16) -> String {
    let mut buffer = String::with_capacity(4);
    loop {
        println!("{}", num % 10);
        let remainder = num % 10;
        num /= 10;
        if remainder == 0 {
            return buffer;
        } else {
            buffer.insert(0, digit_to_sub(remainder));
        }
    }

    fn digit_to_sub(ch: u16) -> char {
        match ch {
            0 => '₀',
            1 => '₁',
            2 => '₂',
            3 => '₃',
            4 => '₄',
            5 => '₅',
            6 => '₆',
            7 => '₇',
            8 => '₈',
            9 => '₉',
            _ => unreachable!("expected one digit"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Identifier;

    #[test]
    fn print_identifier() {
        let ident = Identifier { ch: 'x', index: Some(145) };
        assert_eq!(ident.to_beautiful().as_str(), "x₁₄₅");
    }
}
