mod build;
mod wasm_host;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let s = String::from("ffooo");
        assert_eq!(2 + 2, 4);
    }
}
