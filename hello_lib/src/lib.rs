pub fn get_hello_world() -> String {
    format!("Hello {}", world_lib::get_world())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
