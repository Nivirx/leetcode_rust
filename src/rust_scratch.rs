#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HoursMins {
    total: usize,
}

impl HoursMins {
    fn getHours() -> (usize, usize) {
        unimplemented!();
    }
    
    fn getMins() -> usize {
        unimplemented!();
    }
}

impl From<usize> for HoursMins {

}

impl From<(usize, usize)> for HoursMins {
    
}

fn main() {

}

#[cfg(test)]
mod test {

    #[test]
    fn all_the_tests() {

    }
}