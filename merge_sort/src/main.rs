fn main() {
  let mut arr = [5,9,1,3,4,6,6,3,2];
  merge_sort(&mut arr);
  println!("{:?}", arr);
}

fn merge_sort(arr: &mut [i32]) {

  // if len == 1 nothing to do
  if arr.len() == 1 {
    println!("len 1 nothing to do");
    return;
  }

  // if len == 2 and not sorted switch elements
  if arr.len() == 2 {
    if arr[1] < arr[0] {
      let temp = arr[0];
      arr[0] = arr[1];
      arr[1] = temp;
    }
  }

  // define split point
  let h = arr.len() / 2;

  //call mergesort to sort halfs
  merge_sort(&mut arr[..h]);
  merge_sort(&mut arr[h..]);

  let mut arr2 = vec![0; arr.len()];
  let mut bot = 0;
  let mut top = h;
  let mut f = 0;

  loop {
    if bot < h && top < arr.len() {
      if arr[bot] <= arr[top] {
        arr2[f] = arr[bot];
        bot += 1;
      } else {
        arr2[f] = arr[top];
        top += 1;
      }
    } else if bot < h {
      arr2[f] = arr[bot];
      bot += 1;
    } else if top < arr.len() {
      arr2[f] = arr[top];
      top += 1;
    } else {
      break;
    }
    f += 1;
  }

  f = 0;
  loop {
    arr[f] = arr2[f];
    f += 1;
    if f == arr.len() {break;}
  }
}
