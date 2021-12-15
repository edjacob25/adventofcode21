use super::common::read_file;
use std::fmt::{Debug, Formatter};

pub fn part1() -> usize {
    let path = "test_files/13.txt".to_string();
    let lines = read_file(path);
    let (mut paper, instructions) = create_paper_and_instructions(&lines);
    fold(&mut paper, instructions.get(0).unwrap());
    paper.len()
}

pub fn part2() -> usize {
    let path = "test_files/13.txt".to_string();
    let lines = read_file(path);
    0
}

fn create_paper_and_instructions(lines: &[String]) -> (Vec<Point>, Vec<Fold>){
    let mut i = 0;
    let mut points = Vec::new();
    loop {
        let line = lines.get(i).unwrap();
        if line.is_empty() {
            break;
        }
        let nums = line.split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let point = Point{x:nums[0], y:nums[1]};
        points.push(point);

        i+=1;
    }

    let mut folds = Vec::new();
    for j in i + 1..lines.len() {
        let line = lines.get(j).unwrap();
        let parts = line.split("=").collect::<Vec<_>>();
        let fold = Fold{
            value: parts[1].parse::<u32>().unwrap(),
            fold_type: match parts[0].chars().last().unwrap() {
                'x' => FoldType::Vertical,
                'y' => FoldType::Horizontal,
                _ => panic!("Huh?"),
            }};

        folds.push(fold);
    }
    (points, folds)
}

fn fold(points: &mut Vec<Point>, fold: &Fold) {
    let mut new_points = points.iter()
        .filter(|&p| match fold.fold_type {
            FoldType::Horizontal => p.y >= fold.value,
            FoldType::Vertical => p.x >= fold.value ,
        })
        .map(|p|
            match fold.fold_type {
                FoldType::Horizontal => Point{x:p.x, y: 2 * fold.value - p.y },
                FoldType::Vertical => Point{x: 2 * fold.value - p.x, y: p.y } ,
            }
        ).collect::<Vec<_>>();

    points.retain(|p: &Point| match fold.fold_type {
        FoldType::Horizontal => p.y < fold.value,
        FoldType::Vertical => p.x < fold.value ,
    });

    points.append(&mut new_points);

    points.sort();

    points.dedup();

}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Fold {
    value: u32,
    fold_type: FoldType
}

enum FoldType {
    Horizontal,
    Vertical,
}
