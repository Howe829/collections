use std::collections::HashMap;

const VOWELS:[char;5] = ['a', 'e', 'i', 'o', 'u'];
fn main() {
    let mut numbers = vec![6,6,6,6,1,3,4, 1, 2, 3, 4, 5];
    bubble_sort(&mut numbers);
    let mid = numbers.len() / 2;
    println!("The median of numbers{:?} is: {}", numbers, numbers[mid]);
    
    let mut map = HashMap::new();
    for i in 0..numbers.len(){
        let count = map.entry(numbers[i]).or_insert(0);
        *count +=1;
    }
    let mut max = 0;
    let mut max_value = 0;
    for (key, value) in map{
        if value > max_value {
            max_value = value;
            max = key;
        }
    }
    println!("The mode of the numbers is: {}", max);
    pig_latin(&mut String::from("first"));
    pig_latin(&mut String::from("apple"));
}

fn is_start_with_vowel(my_str:  &String)->bool{
    for i in VOWELS{
        if my_str.starts_with(i){
            return true;
        } 
    }
    return false;
}

fn pig_latin(my_str: &mut String) {
    let mut ends = String::new();
    let mut heads = String::new();
    
    if !is_start_with_vowel(my_str) {
        ends = format!("{}{}", my_str.chars().next().unwrap(), "ay");
        heads = my_str[1..].to_string();
    }else {
        heads = my_str[0..].to_string();
        ends.push_str("hay");
    }
    println!("{}-{}", heads, ends);
}

fn bubble_sort(numbers: &mut Vec<i32>){
   for i in 0..numbers.len(){
       for j in 0..numbers.len(){
           if numbers[i] < numbers[j] {
              let temp = numbers[i];
              numbers[i] = numbers[j];
              numbers[j] = temp;
           }
   
       }    
   }
}


