/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(n: i32) -> u32 {
    let mut length = 1; 
    // ^^^ Ignored since length will be within the scope of a recursive call
    if n == 1 {
        return length
    }
    else if n % 2 == 0 {
        length = dbg!(collatz_length(dbg!(n)/2) + 1); 
        // dbg!() prints out the values to error check
    } 
    else {
        length = dbg!(collatz_length(3*dbg!(n)+1) + 1);
        // dbg!() prints out the values to error check 
    }
    length
  }
  
  fn main() {
    let n = 3;
    println!("collatz_length({n}): {}",collatz_length(n));
  }