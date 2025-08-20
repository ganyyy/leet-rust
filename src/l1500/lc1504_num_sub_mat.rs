struct Tuple {
    j: i32, // 下标
    c: i32, // 矩形的个数
    h: i32, // 高度
}

pub fn num_sub_mat(mat: Vec<Vec<i32>>) -> i32 {
    assert!(!mat.is_empty() && !mat[0].is_empty());
    // 截止到当前行, 连续累计的高度
    let mut heights = vec![0; mat[0].len()];
    // 单调栈, 计算矩形个数
    let mut stack: Vec<Tuple> = Vec::new();

    let mut ret = 0;

    stack.push(Tuple { j: -1, c: 0, h: -1 });
    for row in mat {
        // 保留哨兵
        stack.truncate(1);
        for (j, &val) in row.iter().enumerate() {
            if val == 1 {
                heights[j] += 1;
            } else {
                heights[j] = 0;
            }
        }

        for (j, &height) in heights.iter().enumerate() {
            let j = j as i32;
            // 以当前的高度, 最宽可以到达的位置
            while let Some(top) = stack.last() {
                if top.h < height {
                    break;
                }
                stack.pop();
            }
            // 一定会有一个哨兵
            let last = stack.last().unwrap();
            let (left, mut c) = (last.j, last.c);
            // 可以直接复制的矩形个数 + 当前列带来的额外的矩形个数
            // 现在可以抱枕的是栈顶的高度是一定 < 当前高度的,
            // 那么当前列可以直接扩充之前个数的矩形
            c += (j - left) * height;
            ret += c;
            stack.push(Tuple { j, c, h: height });
        }
    }

    ret
}
