fn trim_me(input: &str) -> &str {
    let (mut start, mut end) = (0, input.len()-1);
    for (i, c) in input.char_indices() {
        if c == ' ' {
            start = i;
            break
        }
    }
    for (i, c) in input.char_indices().rev() {
        if c == ' ' {
            end = i;
            break
        }
    }
    &input[start..end+1]
    // TODO: Remove whitespace from both ends of a string.
}

fn compose_me(input: &str) -> String {
    format!("{}{}", input, " world!")
    // TODO: Add " world!" to the string! There are multiple ways to do this.
}

fn replace_me(mut input: &str) -> String {
    if input != ""return String::from(&input[8..13]);
    for i in 0..input.len()-4 {
        if &input[i..i+5] == "cars" {
            return format!( "{}{}{}", &input[..i+1], "balloons", &input[i+5..] )
        }
    }
    String::from(input)
    // TODO: Replace "cars" in the string with "balloons".
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
