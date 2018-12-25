use std::collections::HashSet;
use itertools::Itertools;

#[macro_use] extern crate scan_fmt;

struct Point(i32, i32);

fn closest_point(p : &Point, points : &Vec<Point>) -> Option<usize> {
    let mut min_dist = 100000;
    let mut min_point = 0;
    let mut unique = false;

    for (i, point) in points.iter().enumerate() {
        let d = dst(p, point);
        if d < min_dist {
            min_dist = d;
            min_point = i;
            unique = true;
        } else if d == min_dist {
            unique = false;
        }
    }
    
    if unique {
        Some(min_point)
    } else {
        None
    }
}

fn dst(p1 : &Point, p2 : &Point) -> i32 {
    (p2.0-p1.0).abs() + (p2.1-p1.1).abs()
}

fn solve_1(points : &Vec<Point>, min_x : i32, max_x : i32, min_y : i32, max_y : i32) {
    let mut area_sizes = vec![0; points.len()];

    for x in min_x .. max_x {
        for y in min_y .. max_y {
            let val = closest_point(&Point(x,y), &points);
            if let Some(i) = val {
                area_sizes[i] += 1
            }
        }
    }

    //Infinite areas are the areas touching the edge of the minimum
    //bounding box
    let mut infinite_areas : HashSet<usize> = HashSet::new();

    let edge_points =
               (min_x .. max_x).map(|x| Point(x, min_y))
        .chain((min_x .. max_x).map(|x| Point(x, max_y)))
        .chain((min_y .. max_y).map(|y| Point(min_x, y)))
        .chain((min_y .. max_y).map(|y| Point(max_x, y)));

    for p in edge_points {
        let val = closest_point(&p, &points);
        if let Some(i) = val {
            infinite_areas.insert(i);
        }
    }

    let res = area_sizes
        .iter()
        .enumerate()
        .filter(|(i,_x)| !infinite_areas.contains(i))
        .max_by_key(|(_i,x)| *x)
        .unwrap();

    println!("{}", res.1);
}

fn dst_to_all_points(p : &Point, points : &Vec<Point>) -> i32 {
    points.iter().map(|point| dst(p,point)).sum()
}

fn solve_2(points : &Vec<Point>, min_x : i32, max_x : i32, min_y : i32, max_y : i32) {
    let res : Vec<i32> = (min_x - 1000 .. max_x + 1000)
        .cartesian_product(min_y - 1000 .. max_y + 1000)
        .map(|(x,y)| dst_to_all_points(&Point(x,y), &points))
        .filter(|x| *x < 10000)
        .collect();

    println!("{}", res.len());
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.lines();

    let mut points = Vec::new();
    for line in input {
        let (x,y) = scan_fmt!(line,
                              "{d}, {d}",
                              i32, i32);
        points.push(Point(x.unwrap(),y.unwrap()));
    }

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (2000,2000,0,0);
    for Point(x,y) in points.iter() {
        if *x < min_x { min_x = *x }
        if *x > max_x { max_x = *x }
        if *y < min_y { min_y = *y }
        if *y > max_y { max_y = *y }
    }
    
    solve_1(&points, min_x, max_x, min_y, max_y);
    solve_2(&points, min_x, max_x, min_y, max_y);
}
