use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    let file = File::open(Path::new("input")).unwrap();
    // We store all the voxels as a hash set
    // We add 1 to every coordinate which will be useful later
    let voxels = io::BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap() + 1)
                .collect_tuple()
                .unwrap()
        })
        .collect::<HashSet<_>>();
    // To compute the surface area, we iterate throught he voxels,
    // and count the number of the 6 directions in which there are no voxels
    let mut surface_area = 0;
    for &(x, y, z) in voxels.iter() {
        if !voxels.contains(&(x + 1, y, z)) {
            surface_area += 1;
        }
        if !voxels.contains(&(x - 1, y, z)) {
            surface_area += 1;
        }
        if !voxels.contains(&(x, y + 1, z)) {
            surface_area += 1;
        }
        if !voxels.contains(&(x, y - 1, z)) {
            surface_area += 1;
        }
        if !voxels.contains(&(x, y, z + 1)) {
            surface_area += 1;
        }
        if !voxels.contains(&(x, y, z - 1)) {
            surface_area += 1;
        }
    }
    println!("Part 1: {}", surface_area);

    // To find the outside surface area we begin by creating a rectangular prism around the voxels
    // We start in one corner of the prism and explore all reachable empty voxels

    // We pad the prism with an empty layer on each side
    let dimensions = voxels
        .iter()
        .fold((0, 0, 0), |(length, width, depth), (x, y, z)| {
            (length.max(x + 2), width.max(y + 2), depth.max(z + 2))
        });
    let mut outside_area = 0;
    // Outside tells us which voxels are lava, nor enclosed by lava
    let mut outside = vec![vec![vec![false; dimensions.2]; dimensions.1]; dimensions.0];
    // We explore the outside depth-first using the outside_stack
    let mut ouside_stack = vec![(0, 0, 0)];
    while let Some((x, y, z)) = ouside_stack.pop() {
        // We always add adject voxels to the stack, even if they've already been visited
        // If they've already been visited, or have lava we skip adding adjacent voxels
        if voxels.contains(&(x, y, z)) {
            // Lava voxels will be reached once per adjacent outside voxel
            // This is how we sum the outside surface area
            outside_area += 1;
            continue;
        }
        if outside[x][y][z] {
            continue;
        }
        outside[x][y][z] = true;
        // These statements add the adjacent voxels as long as they're within the prism
        if x < dimensions.0 - 1 {
            ouside_stack.push((x + 1, y, z));
        }
        if x > 0 {
            ouside_stack.push((x - 1, y, z));
        }
        if y < dimensions.1 - 1 {
            ouside_stack.push((x, y + 1, z));
        }
        if y > 0 {
            ouside_stack.push((x, y - 1, z));
        }
        if z < dimensions.2 - 1 {
            ouside_stack.push((x, y, z + 1));
        }
        if z > 0 {
            ouside_stack.push((x, y, z - 1));
        }
    }
    println!("Part 2: {}", outside_area);
}
