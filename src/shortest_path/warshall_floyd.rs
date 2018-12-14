use common::*;
use std;

// Calculate shortest path by O(n^3) cost.
// If has no edge between x and y and x != y, then assign INF(too large number) to v[x][y].
// If has no edge between x and y and x == y, then assign 0 to v[x][y].
// This algorithm let target grah have negative weighted edges.
pub fn warshall_floyd<T>(v : &mut Vec<Vec<T>>) 
    where T : 
          std::ops::Add<Output = T> + 
          std::cmp::Ord + Copy 
{
    // must be square matrix
    assert!(v.len() == v[0].len());
    let n = v.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                v[i][j] = min(v[i][j], v[i][k] + v[k][j]);
            }
        }
    }
}


#[test]
fn warshll_floyd_test(){
    const INF : i32  = 1 << 29;

    let n = 4;
    let mut v = vec![vec![0 ; n] ; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                v[i][j] = 0;
            }
            else {
                v[i][j] = INF;
            }
        }
    }
    
    v[0][2] = -2;
    v[1][0] =  4; v[1][2] = 3;
    v[2][3] =  2;
    v[3][1] = -1;

    warshall_floyd(&mut v);

    assert_eq!(
        vec![vec![0, -1, -2, 0],
             vec![4, 0, 2, 4],
             vec![5, 1, 0, 2],
             vec![3, -1, 1, 0],
        ],
        v);

}
