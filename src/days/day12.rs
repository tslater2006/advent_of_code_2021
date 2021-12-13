use std::collections::HashMap;

const INPUT: &str = include_str!("..\\..\\inputs\\day12.txt");

pub fn solve_part_1() {
    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in INPUT.lines().into_iter() {
        let parts: Vec<&str> = l.split("-").into_iter().collect();
        match paths.get_mut(parts[0]) {
            Some(p) => {
                p.push(parts[1]);
            }
            None => {
                paths.insert(parts[0], vec![parts[1]]);
            }
        }
        match paths.get_mut(parts[1]) {
            Some(p) => {
                p.push(parts[0]);
            }
            None => {
                paths.insert(parts[1], vec![parts[0]]);
            }
        }
    }
    let mut collected_paths: Vec<Vec<&str>> = vec![];
    find_all_paths(
        &paths,
        "start",
        "end",
        &mut Vec::new(),
        &mut Vec::new(),
        &mut collected_paths,
    );
    println!("Day #12 Part 1: {}", collected_paths.len());
}

pub fn solve_part_2() {
    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in INPUT.lines().into_iter() {
        let parts: Vec<&str> = l.split("-").into_iter().collect();

        match paths.get_mut(parts[0]) {
            Some(p) => {
                p.push(parts[1]);
            }
            None => {
                paths.insert(parts[0], vec![parts[1]]);
            }
        }
        match paths.get_mut(parts[1]) {
            Some(p) => {
                p.push(parts[0]);
            }
            None => {
                paths.insert(parts[1], vec![parts[0]]);
            }
        }
    }

    let mut collected_paths: Vec<Vec<&str>> = vec![];
    find_all_paths2(
        &paths,
        "start",
        "end",
        &mut Vec::new(),
        &mut Vec::new(),
        &mut collected_paths,
        false,
    );

    println!("Day #12 Part 2: {}", collected_paths.len());
}

fn find_all_paths<'a>(
    map: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    end: &str,
    visited: &mut Vec<&'a str>,
    path_list: &mut Vec<&'a str>,
    collected_paths: &mut Vec<Vec<&'a str>>,
) {
    if node == end {
        //dbg!(path_list);
        collected_paths.push(path_list.clone());
        return;
    }

    if node.as_bytes()[0] >= 97 {
        visited.push(node);
    }
    path_list.push(node);

    let children = map.get(node).unwrap();
    for child in children {
        if !visited.contains(child) {
            find_all_paths(
                map,
                child,
                end,
                &mut visited.clone(),
                &mut path_list.clone(),
                collected_paths,
            );
        }
    }
}

fn find_all_paths2<'a>(
    map: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    end: &str,
    visited: &mut Vec<&'a str>,
    path_list: &mut Vec<&'a str>,
    collected_paths: &mut Vec<Vec<&'a str>>,
    has_double_visited: bool,
) {
    /* Each call gets its own clone of the visited list and path list, but share the collected_paths
    collected_paths gets added to when we reach an 'end' cave

    We also track a has_double_visited to indicate if we've been to a particular small cave 2x. If so
    from that point forward we immediately mark any small cave as "visited" which should prevent us from
    going back. Once this is set it is in-effect carried through the rest of the recursive call chain

    */

    /* Check for end condition */
    if node == end {
        path_list.push(node);
        collected_paths.push(path_list.clone());

        return;
    }

    /* copy param to mutable in case it is false and we need to change it */
    let mut did_double_visit = has_double_visited;

    /* dumb hack for "is it lowercase" */
    if node.as_bytes()[0] >= 97 {
        /* if we've already double visited, or we are the start node, mark it visited */
        if has_double_visited || node == "start" {
            visited.push(node);
        } else {
            /* we haven't done a double yet, check to see if this cave is in path list*/
            if path_list.contains(&node) {
                /* it is, so mark us has having just done a double visit, and mark the node as visited */
                did_double_visit = true;
                visited.push(node);
            }
        }
    }

    /* Add ourselves to the path list */
    path_list.push(node);

    /* loop through children, if they have not been visited before call find_all_paths2 on them
    passing clones as needed and the current did_double_visit status */
    let children = map.get(node).unwrap();
    for child in children {
        if did_double_visit && path_list.contains(&child) && child.as_bytes()[0] >= 97 {
            continue;
        }

        if !visited.contains(child) {
            find_all_paths2(
                map,
                child,
                end,
                &mut visited.clone(),
                &mut path_list.clone(),
                collected_paths,
                did_double_visit,
            );
        }
    }
}
