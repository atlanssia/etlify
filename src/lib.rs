mod configs;
mod server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let v = vec![9, 8, 7];
        v.iter().filter(|x| matches!(x, 9));
        println!("{}", v.len());
    }
}
