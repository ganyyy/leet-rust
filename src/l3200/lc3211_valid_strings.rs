struct Solution;

impl Solution {
    /*
        func validStrings(n int) []string {
        var ret []string
        var dfs func(buf []byte)
        dfs = func(buf []byte) {
            if len(buf) == n {
                ret = append(ret, string(buf))
                return
            }
            if len(buf) == 0 || buf[len(buf)-1] == '1' {
                dfs(append(buf, '0'))
            }
            dfs(append(buf, '1'))
        }
        dfs(make([]byte, 0, n))
        return ret
    }
     */

    #[allow(unused)]
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut result = vec![];

        fn dfs(buf: &mut Vec<char>, n: usize, ret: &mut Vec<String>) {
            if buf.len() == n as usize {
                ret.push(buf.iter().collect());
                return;
            }
            if buf.len() == 0 || buf.last().unwrap() == &'1' {
                buf.push('0');
                dfs(buf, n, ret);
                buf.pop();
            }
            buf.push('1');
            dfs(buf, n, ret);
            buf.pop();
        }

        dfs(&mut Vec::with_capacity(n as usize), n as usize, &mut result);

        result
    }
}
