pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![vec![-1; grid[0].len()]; grid.len()];
    let mut ret = 0;
    for row in 0..grid.len() {
        dfs(&mut dp, &mut ret, &grid, 0, row, 0);
    }

    fn dfs(
        dp: &mut Vec<Vec<i32>>,
        ret: &mut i32,
        grid: &Vec<Vec<i32>>,
        cur: i32,
        row: usize,
        col: usize,
    ) {
        let mut it = vec![];
        if row > 0 && col < grid[0].len() - 1 {
            it.push((row - 1, col + 1));
        }
        if col < grid[0].len() - 1 {
            it.push((row, col + 1));
        }
        if row < grid.len() - 1 && col < grid[0].len() - 1 {
            it.push((row + 1, col + 1))
        }
        for (r, c) in it {
            if dp[r][c] == -1 {
                if grid[r][c] > grid[row][col] {
                    dfs(dp, ret, grid, cur + 1, r, c);
                    dp[row][col] = std::cmp::max(dp[r][c], cur);
                    *ret = std::cmp::max(dp[row][col], *ret);
                }
            }
        }
        dp[row][col] = cur;
        *ret = std::cmp::max(dp[row][col], *ret);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn a() {
        assert_eq!(
            max_moves(vec_vec![
                [2, 4, 3, 5],
                [5, 4, 9, 3],
                [3, 4, 2, 11],
                [10, 9, 13, 15]
            ]),
            3
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            max_moves(vec_vec![
                [
                    1000000, 92910, 1021, 1022, 1023, 1024, 1025, 1026, 1027, 1028, 1029, 1030,
                    1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1040, 1041, 1042, 1043,
                    1044, 1045, 1046, 1047, 1048, 1049, 1050, 1051, 1052, 1053, 1054, 1055, 1056,
                    1057, 1058, 1059, 1060, 1061, 1062, 1063, 1064, 1065, 1066, 1067, 1068
                ],
                [
                    1069, 1070, 1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1079, 1080, 1081,
                    1082, 1083, 1084, 1085, 1086, 1087, 1088, 1089, 1090, 1091, 1092, 1093, 1094,
                    1095, 1096, 1097, 1098, 1099, 1100, 1101, 1102, 1103, 1104, 1105, 1106, 1107,
                    1108, 1109, 1110, 1111, 1112, 1113, 1114, 1115, 1116, 1117, 1118
                ]
            ]),
            49
        );
    }
}
