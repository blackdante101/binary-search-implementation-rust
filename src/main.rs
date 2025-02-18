fn main() {
    let data = [0,3,7,9,16,34,56,78,90];
    let target: i32 = 78;
    
    binary_search(&data, target);
   
}


fn binary_search(data:&[i32],  target:i32){

    let mut low = 0;
    let mut high = data.len()-1;

    while low <= high {
        let mid = low + (high -low)/2;

        if data[mid] == target {
            println!("We've found the target: {}", data[mid]);
            break;
        }else if data[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
}