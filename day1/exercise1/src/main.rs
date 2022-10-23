fn main() {
    println!("Hello, world!");
    let org_arr = [1,2,3,5,6,8,10,11];
    let sub_arr = [4,6,8,10];
    let mut i =0;
    let mut j =0;
    let mut count = 0;
    while i < org_arr.len() {
        while j < sub_arr.len() {
            if org_arr [i] == sub_arr[j] {
                count = count + 1;
            } else {
                count = 0;
                break;
            }
            i = i + 1;
            j = j + 1;
        }
        i = i + 1;
        println!("This asd {}", count);
        if count == sub_arr.len() {
            println!("This is sub array");
            break;
        } if i  == (org_arr.len() - 1)  && count < sub_arr.len(){
            println!("This is not sub array");
            break; 
        }
    }
}

