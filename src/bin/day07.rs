use std::io::BufRead;

use anyhow::{Result, bail};

struct Filesystem {
    nodes: Vec<Node>
}

impl Filesystem {
    fn insert(&mut self, parent: Option<usize>, fs_node: FsNode) -> usize {
        let inode = self.nodes.len();
        self.nodes.push(Node {
            inode,
            fs_node,
            parent,
            children: vec![]
        });
        if let Some(p_inode) = parent {
            self.nodes[p_inode].children.push(inode);
        }
        inode
    }
    fn get(&self, idx: usize) -> Option<&Node> {
        self.nodes.get(idx)
    }
}

struct Node {
    inode: usize,
    fs_node: FsNode,
    parent: Option<usize>,
    children: Vec<usize>
}

enum FsNode {
    File {
        name: String,
        size: usize
    },
    Directory {
        name: String
    }
}

fn directory_sizes(fs: &Filesystem, root: usize, sizes: &mut Vec<usize>) -> usize {
    let mut sum = 0;
    for ic in &fs.get(root).unwrap().children {
        let c = fs.get(*ic).unwrap();
        match &c.fs_node {
            FsNode::File { name, size } => {
                sum += size;
            },
            FsNode::Directory { name } => {
                sum += directory_sizes(fs, *ic, sizes);
            }
        }
    }
    sizes.push(sum);
    sum
}

fn solve<T: BufRead>(input: T) -> Result<Vec<usize>> {
    let mut fs = Filesystem {
        nodes: vec![]
    };
    let iroot = fs.insert(None, FsNode::Directory{ name: "".into() });
    let mut ipwd = iroot;
    for line in input.lines() {
        let l = line?;
        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts[0] == "$" {
            match parts[1] {
                "cd" => {
                    if parts[2] == "/" {
                        ipwd = iroot;
                    } else if parts[2] == ".." {
                        ipwd = fs.get(ipwd).unwrap().parent.unwrap();
                    } else {
                        for c in &fs.get(ipwd).unwrap().children {
                            if let FsNode::Directory{name} = &fs.get(*c).unwrap().fs_node {
                                if name == parts[2] {
                                    ipwd = *c;
                                    break;
                                }
                            }
                        }
                    }
                }
                "ls" => {}
                _ => {
                    bail!("Invalid command {}", parts[1]);
                }
            }
        } else {
            let n = if parts[0] == "dir" {
                FsNode::Directory { name: parts[1].into() }
            } else {
                FsNode::File { name: parts[1].into(), size: parts[0].parse()? }
            };
            fs.insert(Some(ipwd), n);
        }
    }
    let mut buf = Vec::new();
    let total_size = directory_sizes(&fs, iroot, &mut buf);
    let sol1: usize = buf.iter().filter(|x| **x <= 100000).sum();
    buf.sort();
    let sol2 = buf.iter().find(|x| 70000000 - total_size + **x >= 30000000).unwrap();

    Ok(vec![
       sol1,
       *sol2
    ])
}

fn main() -> Result<()> {
    for (i, s) in solve(std::io::stdin().lock())?.iter().enumerate() {
        println!("part {}: {}", i+1, s);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(solve(include_bytes!("../../data/day07_example.txt").as_slice()).unwrap(), [95437, 24933642]);
    }
    #[test]
    fn input() {
        assert_eq!(solve(include_bytes!("../../data/day07_input.txt").as_slice()).unwrap(), [1084134, 6183184]);
    }
}
