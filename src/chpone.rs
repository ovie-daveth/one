pub fn chpone(){

    let sum = testrun(2, 5);

    let arrray_1 = [1,2,3,4,5];
    let tuple_1 = (23, "me", false);
    

    for i in arrray_1.iter(){
        println!("The one {}", i);
    }

    let mut number = arrray_1.len();

    while number != 0 {
        println!("The other one {}", arrray_1[number-1]);
        
        number -= 1;
    }


    let (v1, v2, v3) = tuple_1;

    println!("v1, v2, v3: {}, {}, {}", v1, v2, v3);
    println!("The value of v1 {}", tuple_1.1);
    println!("The sum is {}", sum);

    fn testrun(num: i32, num2: i32) -> i32
    {
        return num + num2
    }
}