#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let sum = 1 + 5;
        assert_eq!(sum, 6);
    }

    #[test]
    fn test2() {
        let str1 = "hello";
        assert_eq!(str1, "hi");
    }
}
