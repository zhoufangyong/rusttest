﻿fn sum(list: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in list.into_iter() {
        match sum.checked_add(*i) {
            Some(data) => sum = data,
            None => return  None
        }
    }
    Some(sum)
}
fn main() {

    //正常计算，输出和
    let list = vec![1u32,2,3,4,5];
    println!("计算结果（数字）{}",sum(&list).unwrap());

    //正常计算，输出Option
    let list2 = vec![1u32,2,3,4,5];
    let result2: Option<u32> = sum(&list2);
    println!("计算结果（枚举）:{:?}", result2);
    
    //溢出，None
    let list3 = vec![u32::max_value()-1, 2];
    let result3: Option<u32> = sum(&list3);
    println!("溢出结果（None）:{:?}", result3);

}