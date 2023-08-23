macro_rules! fn_runner {
    ($($function:ident),*)=>{
           (
               $($function()),*
           )
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use proc_macros::proc_fn_runner;

    fn fo() -> i32 {
        2
    }

    fn foo() -> f32 {
        4.2
    }

    fn fooo() -> String {
        String::from("testing")
    }

    #[test]
    fn declarative_macro_success() {
        let expected = (2, 4.2f32, String::from("testing"));
        let actual = fn_runner!(fo, foo, fooo);
        assert_eq!(expected, actual)
    }

    #[test]
    fn procedural_macro_success() {
        let expected = (2, String::from("testing"));
        let actual = proc_fn_runner!("fo", "foo", "fooo");
        assert_eq!(expected, actual)
    }
}
