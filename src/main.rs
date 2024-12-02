
fn process_lists<T: PartialOrd + Copy + Ord + Into<i32> + From<i32>>(list1: &mut Vec<T>, list2: &mut Vec<T>) -> i32 {

    let mut total_difference = 0;

    while !list1.is_empty() && !list2.is_empty() {
        let min_list1 = *list1.iter().min().unwrap();
        let min_list2 = *list2.iter().min().unwrap();

        let min_list1: i32 = min_list1.into();
        let min_list2: i32 = min_list2.into();

        let difference = (min_list1 - min_list2).abs();

        total_difference += difference;

        if let Some(index) = list1.iter().position(|&x| x == min_list1.into()) {
            list1.remove(index); 
        }

        if let Some(index) = list2.iter().position(|&x| x == min_list2.into()) {
            list2.remove(index); 
        }
    }

    total_difference
            
}


fn main() {
    let mut list1 = vec![3,4,2,1,3,3];
    let mut list2 = vec![4,3,5,3,9,3];


    let result = process_lists(&mut list1, &mut list2);


    println!("Total difference: {}", result);

}
