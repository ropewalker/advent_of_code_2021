use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl From<(i32, i32, i32)> for Vec3 {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Vec3 { x, y, z }
    }
}

impl From<Vec3> for (i32, i32, i32) {
    fn from(vec3: Vec3) -> Self {
        (vec3.x, vec3.y, vec3.z)
    }
}

type Beacon = Vec3;

impl From<&str> for Beacon {
    fn from(coordinates_str: &str) -> Self {
        let mut coordinates = coordinates_str
            .split(',')
            .map(|s| s.parse::<i32>().unwrap());

        Self {
            x: coordinates.next().unwrap(),
            y: coordinates.next().unwrap(),
            z: coordinates.next().unwrap(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Scanner(HashSet<Beacon>);

impl From<&str> for Scanner {
    fn from(scanner_str: &str) -> Self {
        Scanner(scanner_str.lines().skip(1).map(|l| l.into()).collect())
    }
}

impl Scanner {
    fn overlap_regions_with(&self, another_scanner: &Scanner) -> (HashSet<Beacon>, Scanner, Vec3) {
        for transformed_scanner in another_scanner.all_transformations() {
            for beacon in self.0.iter() {
                for another_beacon in transformed_scanner.0.iter() {
                    let updated_scanner = Scanner(
                        transformed_scanner
                            .0
                            .iter()
                            .map(|b| {
                                (
                                    b.x + beacon.x - another_beacon.x,
                                    b.y + beacon.y - another_beacon.y,
                                    b.z + beacon.z - another_beacon.z,
                                )
                                    .into()
                            })
                            .collect(),
                    );

                    if self.0.intersection(&updated_scanner.0).count() >= 12 {
                        let result = updated_scanner.0.union(&self.0).cloned().collect();
                        return (
                            result,
                            updated_scanner,
                            (
                                beacon.x - another_beacon.x,
                                beacon.y - another_beacon.y,
                                beacon.z - another_beacon.z,
                            )
                                .into(),
                        );
                    }
                }
            }
        }

        (
            HashSet::with_capacity(0),
            another_scanner.to_owned(),
            (0, 0, 0).into(),
        )
    }

    fn all_transformations(&self) -> Vec<Scanner> {
        Vec::from([
            Scanner(self.0.iter().map(|b| (b.x, b.y, b.z).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.x, b.z, -b.y).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.x, -b.y, -b.z).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.x, -b.z, b.y).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.x, b.y, -b.z).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.x, b.z, b.y).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.x, -b.y, b.z).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.x, -b.z, -b.y).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.y, b.x, -b.z).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.y, -b.x, b.z).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.y, b.z, b.x).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.y, -b.z, -b.x).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.y, b.x, b.z).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.y, -b.x, -b.z).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.y, b.z, -b.x).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.y, -b.z, b.x).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.z, b.x, b.y).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.z, -b.x, -b.y).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.z, b.y, -b.x).into()).collect()),
            Scanner(self.0.iter().map(|b| (b.z, -b.y, b.x).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.z, b.x, -b.y).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.z, -b.x, b.y).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.z, b.y, b.x).into()).collect()),
            Scanner(self.0.iter().map(|b| (-b.z, -b.y, -b.x).into()).collect()),
        ])
    }
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(|s| s.into()).collect()
}

#[aoc(day19, part1)]
fn part1(scanners: &[Scanner]) -> usize {
    let mut beacons = HashSet::new();
    let mut scanners = scanners.to_owned();

    let mut adjusted_scanners = vec![scanners.pop().unwrap()];

    while !scanners.is_empty() {
        let mut non_overlapping = Vec::new();

        let adjusted_scanner = adjusted_scanners.pop().unwrap();

        for scanner in scanners.into_iter() {
            let (new_beacons, scanner, _) = adjusted_scanner.overlap_regions_with(&scanner);

            if !new_beacons.is_empty() {
                adjusted_scanners.push(scanner);
                beacons.extend(new_beacons);
            } else {
                non_overlapping.push(scanner);
            }
        }

        scanners = non_overlapping;
    }

    beacons.len()
}

#[aoc(day19, part2)]
fn part2(scanners: &[Scanner]) -> i32 {
    let mut scanners = scanners.to_owned();

    let mut adjusted_scanners = vec![scanners.pop().unwrap()];
    let mut distances = vec![Vec3 { x: 0, y: 0, z: 0 }];

    while !scanners.is_empty() {
        let mut non_overlapping = Vec::new();

        let adjusted_scanner = adjusted_scanners.pop().unwrap();

        for scanner in scanners.into_iter() {
            let (new_beacons, scanner, distance) = adjusted_scanner.overlap_regions_with(&scanner);

            if !new_beacons.is_empty() {
                adjusted_scanners.push(scanner);
                distances.push(distance);
            } else {
                non_overlapping.push(scanner);
            }
        }

        scanners = non_overlapping;
    }

    let mut result = 0;

    for (i, distance1) in distances.iter().enumerate() {
        for distance2 in distances.iter().skip(i) {
            result = result.max(
                (distance1.x - distance2.x).abs()
                    + (distance1.y - distance2.y).abs()
                    + (distance1.z - distance2.z).abs(),
            )
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 79);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 3_621);
    }
}
