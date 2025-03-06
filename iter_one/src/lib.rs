#[derive(Debug, PartialEq)]
pub struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()   
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::result;


    #[test]
    fn iter_sum() {
        let v = vec![1, 2, 3, 4, 5];

        let v_iter = v.iter();

        let total: i32 = v_iter.sum();

        assert!(total == 15);
    }

    #[test]
    fn iter_map() {
        let v = vec![1, 2, 3, 4, 5];

        let result: Vec<i32> = v.iter().map(|x| x + 1).collect();

        assert!(result == vec![2, 3, 4, 5, 6]);

        dbg!(v);

    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let shoe_size = 10;

        let in_my_size = shoes_in_size(shoes, shoe_size);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }

}