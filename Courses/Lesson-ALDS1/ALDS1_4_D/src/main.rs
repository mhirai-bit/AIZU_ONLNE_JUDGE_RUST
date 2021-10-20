//参考 https://blog.manuel1024.com/archives/143
// 積み込み可能な重さPを、最大値から二分探索で減らして行くことで高速に
// 解を求めている

fn main() {
    let mut n_k = String::new();

    std::io::stdin().read_line(&mut n_k).unwrap();

    let mut iter = n_k.split_whitespace();
    let n: u64 = iter.next().unwrap().parse().unwrap();
    let k: u64 = iter.next().unwrap().parse().unwrap();

    let mut w = vec![];

    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();    
        w.push(input.trim().parse::<u64>().unwrap());
    }

    let mut ac; //荷物をk台のトラックに積み切れるような最大積載量P
    let mut wa = 0; //荷物をk台のトラックに積み切れないような最大積載量P
    ac = w.iter().sum(); // 荷物の重さの挿話がacの初期値

    //二分探索で条件を満たす最大積載量Pの最小値を求める
    while ac - wa > 1 {
        let mid = (wa + ac)/2; //最大積載量Pをmidと仮定する
        let mut isac: bool = true; //荷物を積みきれるならtrue, 積みきれないならfalse
        let mut nowk = 1; //nowk大目のトラックに荷物を積み込む途中
        let mut now_w = 0; //nowk大目のトラックに積載済みの荷物の重さはnow_w

        // 荷物をk台のトラックに積み切れるか判定する
        for i in 0..n as usize {
            //nowk大目のトラックに荷物を積めるか判定
            if now_w + w[i] <= mid {
                now_w += w[i];
            } else {
                nowk += 1; //荷物を積めないならあきらめて次のトラックに積む
                if w[i] <= mid {
                    now_w = w[i];
                } else {
                    //荷物1個の重さがPを超えたらダメ
                    isac = false;
                    break;
                }
            }
        }
        if nowk > k {
            isac = false; //使うトラックの台数がkを超えたらダメ
        }
        if isac {
            ac = mid; // トラックの台数P以内で候補のmidを最小値として積み込み可能だった場合、
                      // 次のループではmidを減らして再度積み込み可能か確認する
        } else {
            wa = mid; // トラックの台数P以内で積み込み不可能だった場合、
                      //　midを増加させて再度loopする
        }
    }

    println!("{}", ac);
}
