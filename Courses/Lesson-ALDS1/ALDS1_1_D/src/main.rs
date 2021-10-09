// fn main() {
//     let mut num_of_input = String::new();
//     std::io::stdin().read_line(&mut num_of_input).unwrap();
//     let num_of_input = num_of_input.trim().parse::<u64>().unwrap();
//     let mut nums = vec![];
//     let mut result = std::i64::MIN;

//     for _ in 0..num_of_input {
//         let mut input = String::new();
//         std::io::stdin().read_line(&mut input).unwrap();
//         let num = input.trim().parse::<i64>().unwrap();
//         if nums.len() == 0 {
//             nums.push(num);
//             continue;
//         }
//         if num - nums.iter().min().unwrap() > result {
//             result = num - nums.iter().min().unwrap() ;
//         }
//         nums.push(num); 
//     }

//     println!("{}", result);
// }


use std::io;

fn main() {
    let mut tmp_str = String::new();
    // 標準入力から一行受け取る
    io::stdin().read_line( &mut tmp_str );

    // 標準入力から受け取った文字を数字に変換する
    let linenum: i32 = tmp_str.trim().parse().expect( "expected val is inreger" );

    // 動的配列を宣言
    let mut numv: Vec<i32> = Vec::new();
    for i in 0..linenum {
        // enter区切りで標準入力から数値を受け取り, 動的配列の末尾へpushする
        tmp_str = String::new();
        io::stdin().read_line( &mut tmp_str );
        let tmp_val = tmp_str.trim().parse().expect( "Failed read line from standard input" );
        numv.push( tmp_val );
    }

    let mut max_val = -1000000000;
    let mut min_val = numv[0];
    for i in 1..linenum {
        if max_val < numv[i as usize] - min_val {
            max_val = numv[i as usize] - min_val;
        }

        if min_val > numv[i as usize] {
            min_val = numv[i as usize];
        }
    }
    println!( "{}", max_val );
}
