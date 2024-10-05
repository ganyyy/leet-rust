#[allow(unused)]
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    assert_eq!(gas.len(), cost.len());
    let (mut start, mut run, mut rest) = (0i32, 0, 0);
    gas.iter()
        .zip(cost.iter())
        .enumerate()
        .for_each(|(position, (gas, cost))| {
            let add = gas - cost;
            rest += add;
            run += add;
            run.lt(&0).then(|| {
                start = (position + 1) as i32;
                run = 0;
            });
        });
    return rest.lt(&0).then(|| -1).unwrap_or(start);
}
