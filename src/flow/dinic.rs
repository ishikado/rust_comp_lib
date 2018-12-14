use common::*;
use std::collections::VecDeque;


const INF : i64 = 1 << 60;

#[derive(Clone)]
struct Edge {
    to  : usize,
    rev : usize,
    cap : i64
}

pub struct Dinic {
    g     : Vec<Vec<Edge>>,
    level : Vec<i32>,
    iter  : Vec<usize>
}

// calc max flow O(V^2*E)
impl Dinic{
    pub fn new(n : usize) -> Dinic {
        let mut d = Dinic{g : vec![], level : vec![], iter : vec![]};
        d.init(n);
        return d;
    }

    fn init(&mut self, n : usize) {
        self.g     = vec![vec![Edge{to : 0, rev : 0, cap : 0} ; 0] ; n];
        self.level = vec![-1 ; n];
        self.iter  = vec![0  ; n]
    }

    pub fn add_edge(&mut self, from : usize, to : usize, cap : i64) {
        let rev1 = self.g[to].len();
        let rev2 = self.g[from].len();
        self.g[from].push(Edge{to : to,   cap : cap, rev : rev1});
        self.g[to].push(  Edge{to : from, cap : 0,   rev : rev2})
    }
    fn bfs(&mut self, s : usize){
        for i in 0..self.g.len() {
            self.level[i] = -1;
        }
        let mut que : VecDeque<usize> = VecDeque::new();
        self.level[s] = 0;
        que.push_back(s);
        while !que.is_empty() {
            let v = *(que.front().unwrap());
            que.pop_back();
            for i in 0..self.g[v].len() {
                let e = self.g[v][i].clone();
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    que.push_back(e.to);
                }
            }
        }
    }
    fn dfs(&mut self, v : usize, t : usize , f : i64) -> i64 {
        if v == t {
            return f;
        }
        for i in self.iter[v]..self.g[v].len() {
            let e = &self.g[v][i].clone();
            if e.cap > 0 && self.level[v] < self.level[e.to] {
                let d = self.dfs(e.to, t, min(f, e.cap));
                if d > 0 {
                    self.g[v][i].cap        -= d;
                    self.g[e.to][e.rev].cap += d;
                    return d;
                }
            }
        }
        return 0;
    }
    pub fn max_flow(&mut self, s : usize, t : usize) -> i64{
        let mut flow = 0;
        loop {
            self.bfs(s);
            if self.level[t] < 0 {
                return flow;
            }
            for i in 0..self.iter.len() {
                self.iter[i] = 0;
            }
            loop {
                let f = self.dfs(s, t, INF);
                if !(f > 0) {
                    break;
                }
                flow += f;
            }
        }
    }
}


#[test]
fn dinic_test(){

    let mut d = Dinic::new(5);
    d.add_edge(0, 1, 5);
    d.add_edge(0, 2, 2);
    d.add_edge(0, 3, 8);

    d.add_edge(1, 2, 3);
    d.add_edge(1, 4, 4);

    d.add_edge(2, 3, 4);
    d.add_edge(2, 4, 4);

    d.add_edge(3, 4, 6);


    let result   = d.max_flow(0, 4);
    let expected = 12;
    assert_eq!(expected, result);
}
