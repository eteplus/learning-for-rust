// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let str1 = String::from("Foo");
        let str2 = String::from("Foo");
        assert_eq!(str1, str2);
    }
}
