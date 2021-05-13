use rand;

fn drop_static<T: 'static>(t: T) {
    std::mem::drop(t);
}


#[cfg(test)]
mod test {
    use crate::static_bound_demo::drop_static;

    #[test]
    fn test_static_bound() {
        let mut strings: Vec<String> = Vec::new();
        for _ in 0..10 {
            if rand::random() {
                // all the strings are randomly generated
                // and dynamically allocated at run-time
                let string = rand::random::<u64>().to_string();
                strings.push(string);
            }
        }

        // strings are owned types so they're bounded by 'static
        for mut string in strings {
            // all the strings are mutable
            string.push_str("a mutation");
            // all the strings are droppable
            drop_static(string); // compiles
        }

        // all the strings have been invalidated before the end of the program
        println!("i am the end of the program");
    }
}