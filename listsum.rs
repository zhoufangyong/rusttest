fn ListSum(list: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for item in list.iter(){
        if sum.checked_add(item*1) == None{
            //println!("overflow");
            return None;
        }    
        sum+=item;
    }
    //println!("sum:{}",sum);
    Some(sum)
}

fn main() {

    //正常计算，输出和
    let list = vec![1u32,2,3,4,5];
    println!("计算结果（数字）{}",ListSum(&list).unwrap());

    //正常计算，输出Option
    let list2 = vec![1u32,2,3,4,5];
    let result2: Option<u32> = ListSum(&list2);
    println!("计算结果（枚举）:{:?}", result2);
    
    //溢出，None
    let list3 = vec![u32::max_value()-1, 2,3,4,5];
    let result3: Option<u32> = ListSum(&list3);
    println!("溢出结果（None）:{:?}", result3);

}
