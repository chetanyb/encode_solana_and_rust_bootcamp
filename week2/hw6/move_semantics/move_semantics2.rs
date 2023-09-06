// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    
    /*Method 1: cloning vec0 into a new vector and passing that to fill_vec function
    let vecNew = vec0.clone();
    let mut vec1 = fill_vec(vecNew); */

    /*Method 2: borrowing vec0 and passing it to fill_vec function
    let mut vec1 = fill_vec(&vec0);
    CONTINUED IN fill_vec() below */

    //Method 3(including comments): creating a mutable reference to vec0 and passing it to fill_vec function
    let mut vec0: Vec<i32> = Vec::new();
    fill_vec(&mut vec0);
    //let mut vec1 = fill_vec(&mut vec0);
    //CONTINUED IN fill_vec() below

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    //vec1.push(88);

    //println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

/*Method 2 continued

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vecNew = Vec::new();

    for item in vec {
        vecNew.push(*item);
    }

    vecNew.push(22);
    vecNew.push(44);
    vecNew.push(66);

    vecNew
} */

//Method 3 continued(including comments)
fn fill_vec(vec: &mut Vec<i32>) /*-> Vec<i32>*/ {
    //let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    //vec
}
