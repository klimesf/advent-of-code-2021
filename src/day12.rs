use std::collections::{HashMap, VecDeque};

use crate::io::read_lines;

struct Vertex {
    edges: Vec<String>,
    small: bool,
}

struct Explorable {
    name: String,
    path: Vec<String>,
}

pub(crate) fn day12() {
    let filename = "input/day12/input.txt";

    let mut vertices: HashMap<String, Vertex> = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(edge) = line {
                let mut parts = edge.split("-");
                let start = parts.next().unwrap().to_string();
                let end = parts.next().unwrap().to_string();

                let start_vertex = vertices.entry(start.clone()).or_insert(Vertex {
                    edges: vec!(),
                    small: is_small(start.clone()),
                });
                start_vertex.edges.push(end.clone());

                let end_vertex = vertices.entry(end.clone()).or_insert(Vertex {
                    edges: vec!(),
                    small: is_small(end.clone()),
                });
                end_vertex.edges.push(start.clone());
            }
        }
    }

    part_a(&mut vertices);
    part_b(&mut vertices);
}

fn part_a(vertices: &mut HashMap<String, Vertex>) {
    let mut to_explore: VecDeque<Explorable> = VecDeque::new();
    let mut paths: Vec<Vec<String>> = vec!();
    to_explore.push_back(Explorable { name: "start".to_string(), path: vec!() });

    while !to_explore.is_empty() {
        let e = to_explore.pop_front().unwrap();
        let vertex_name = e.name;

        // If we hit end, we finish the exploration
        if vertex_name == "end" {
            let mut path = e.path.clone();
            path.push("end".to_string());
            paths.push(path);
            continue
        }

        let vertex = vertices.get(vertex_name.as_str()).unwrap();
        let mut path = e.path.clone();
        path.push(vertex_name.clone());

        for dest in &vertex.edges {
            if path.contains(dest) && vertices.get(dest).unwrap().small {
                continue; // Do not visit small caves twice
            }
            to_explore.push_back(Explorable { name: dest.clone(), path: path.clone() });
        }
    }

    println!("{}", paths.len());
}

fn part_b(vertices: &mut HashMap<String, Vertex>) {
    let mut to_explore: VecDeque<Explorable> = VecDeque::new();
    let mut paths: Vec<Vec<String>> = vec!();
    to_explore.push_back(Explorable { name: "start".to_string(), path: vec!() });

    let contains_small_only_once = |path: &Vec<String>, dest: String| {
        let mut counter: HashMap<String, i32> = HashMap::new();
        counter.insert(dest, 1);
        for edge in path {
            if vertices.get(edge).unwrap().small {
                let cnt = counter.entry(edge.to_string()).or_insert(0);
                *cnt += 1;

                if *cnt > 2 {
                    return false;
                }
            }
        }
        let small_doubles_count = counter.values().into_iter()
            .filter(|v| **v > 1)
            .count();
        return small_doubles_count < 2;
    };

    while !to_explore.is_empty() {
        let e = to_explore.pop_front().unwrap();
        let vertex_name = e.name;

        // If we hit end, we finish the exploration
        if vertex_name == "end" {
            let mut path = e.path;
            path.push("end".to_string());
            paths.push(path);
            continue
        }

        let vertex = vertices.get(vertex_name.as_str()).unwrap();
        let mut path = e.path;
        path.push(vertex_name.clone());

        for dest in &vertex.edges {
            if dest == "start" {
                continue; // Never visit start twice
            }
            if vertices.get(dest).unwrap().small && !contains_small_only_once(&path, dest.clone()) {
                continue; // Do not visit small caves twice
            }
            to_explore.push_back(Explorable { name: dest.clone(), path: path.clone() });
        }
    }

    // for path in &paths {
    //     println!("{}", path.iter().join(","));
    // }
    println!("{}", paths.len());
}

fn is_small(s: String) -> bool {
    for c in s.chars() {
        if c.is_uppercase() {
            return false;
        }
    }
    return true;
}
