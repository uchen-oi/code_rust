fn main() {
    let mut vec=vec![3,1,4,1,5,9,2,6];
    vec.sort();
    println!("{:?}",vec);
}
// fn quick_sort<T>(arr:&mut [T]) where T:std::cmp::Ord {
//     if arr.len()<=1{
//         return;
//     }
//     let pivot=arr[arr.len()/2];
//     let (left,right)=arr.partition_with(|&x|x<pivot);
//     quick_sort(&mut left);
//     quick_sort(&mut right);
// }
// fn main() {
//     let mut vec=vec![3,1,4,1,5,9,2,6];
//     quick_sort(&mut vec);
//     println!("{:?}",vec);
// }

