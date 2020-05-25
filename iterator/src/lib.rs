#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}


fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // closure captures shoe_size from environment
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// Counter struct that could iterate from 1 to 5
struct Counter {
    count: u32,
}

// constructor for Counter
impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

// Iterator trait for Counter
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new() // Counter that could iterate from 1 to 5
            .zip(Counter::new().skip(1)) // zip() produces 4 pairs: (1, 2), (2, 3), (3, 4), (4, 5) (return None instead of (5, None))
            .map(|(a, b)| a * b) // map() to 2, 6, 12, 20
            .filter(|x| x % 3 == 0) // filter() to 6 and 12 which are divisible by 3
            .sum();

        assert_eq!(18, sum);
    }
}
