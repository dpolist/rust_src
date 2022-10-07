use std::collections::HashMap;

pub fn median (mut vector: Vec<i32>)  -> i32 {
    vector.sort();
    let size = vector.len() / 2;
    vector[size]
}

pub fn mode(vector : Vec<i32>) -> i32 {
    let mut frequencies = HashMap::new();
    for item in vector {
        if let std::collections::hash_map::Entry::Occupied(_) = frequencies.entry(item) {
            let curval = frequencies.get(&item).unwrap();
            frequencies.insert(item, curval+1)
        } else {
            frequencies.insert(item, 1)
        };
    }   
    
    let mut mode_item: i32 = 0;
    let mut max_freq: i32 = 0;
    for (item,freq) in &frequencies {
        if *freq > max_freq {
            mode_item = *item;
            max_freq = *freq;
        }
    }
    mode_item
}