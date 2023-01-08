mod errors;
mod trees;

use errors::{ Error, ParsingError };
use itertools::{ Itertools, FoldWhile::{ Continue, Done } };
use std::{ fs::File, io::{ BufRead, BufReader } };
use trees::{ Forest, Tree, TreeVisibility };

fn parse_line(line: String) -> Result<Vec<Tree>, Error> {
    let char_to_tree = |character: char| -> Result<Tree, Error> {
        character.to_digit(10)
        .ok_or(Error::from(ParsingError::InvalidTreeHeight(character)))
        .map(|height| Tree::new(height as u8))
    };

    line.chars().map(char_to_tree).collect()
}

fn parse_input_lines<IterType, IterError>(lines: IterType) -> Result<Forest, Error>
where IterType: Iterator<Item = Result<String, IterError>>
    , Error: From<IterError>
{
    let rows: Result<Vec<_>, _> = lines
        .map(|line_result| line_result.map_err(Error::from).and_then(parse_line))
        .collect();

    Ok(Forest { rows: rows? })
}

fn read_input(path: &str) -> Result<Forest, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_input_lines(reader.lines())
}

fn compute_visibility_for_sequence<'a, IterType>(sequence_iter: IterType, visibility: TreeVisibility)
where IterType: Iterator<Item = &'a mut Tree>
{
    let mut max_height: Option<u8> = None;
    let mut is_visible;
    for tree in sequence_iter {
        (is_visible, max_height) = match max_height {
            Some(max_height) => (max_height < tree.height, Some(std::cmp::max(max_height, tree.height))),
            None => (true, Some(tree.height))
        };
        
        if is_visible {
            tree.visibility.insert(visibility);
        }
        else {
            tree.visibility.remove(visibility);
        }
    }
}

fn compute_visibility(forest: &mut Forest) {
    for row_idx in 0..forest.height() {
        compute_visibility_for_sequence(forest.iter_row(row_idx), TreeVisibility::West);
        compute_visibility_for_sequence(forest.iter_row(row_idx).rev(), TreeVisibility::East);
    }

    for col_idx in 0..forest.width() {
        compute_visibility_for_sequence(forest.iter_col(col_idx), TreeVisibility::North);
        compute_visibility_for_sequence(forest.iter_col(col_idx).rev(), TreeVisibility::South);
    }
}

fn count_visible_trees_from<'a, IterType>(mut trees: IterType, source_height: u8) -> u32 
where IterType: Iterator<Item = &'a mut Tree> + ExactSizeIterator<Item = &'a mut Tree>
{
    trees.fold_while(0 as u32, |count, tree| {
        if tree.height < source_height {
            Continue(count + 1)
        }
        else {
            Done(count + 1)
        }
    })
    .into_inner()
}

fn compute_scenic_score(forest: &mut Forest) {
    for (row, col) in Itertools::cartesian_product(0..forest.height(), 0..forest.height()) {
        let tree_height = forest.rows[row][col].height;
        let scenic_score = 
            count_visible_trees_from(forest.left_of(row, col), tree_height)
            * count_visible_trees_from(forest.right_of(row, col), tree_height)
            * count_visible_trees_from(forest.top_of(row, col), tree_height)
            * count_visible_trees_from(forest.bottom_of(row, col), tree_height);
        
        forest.rows[row][col].scenic_score = scenic_score;
    }
}

fn count_visible_trees(forest: &Forest) -> usize {
    forest.rows
        .iter()
        .flatten()
        .filter(|tree| !tree.visibility.is_empty())
        .count()
}

fn find_max_visibility_score(forest: &Forest) -> Option<u32> {
    forest.rows
        .iter()
        .flatten()
        .map(|tree| tree.scenic_score)
        .max()
}

fn main() {
    match read_input("inputs/2022/08/TreeTopTreeHouse.txt") {
        Ok(mut forest) => {
            compute_visibility(&mut forest);
            compute_scenic_score(&mut forest);
            let solution1 = count_visible_trees(&forest);
            let solution2 = find_max_visibility_score(&forest).unwrap_or_default();
            println!("Solution 1: {solution1}");
            println!("Solution 2: {solution2}");
        },
        Err(err) => println!("{err:?}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn mock_trees_from_heights(heights: Vec<u8>) -> Vec<Tree> {
        heights.into_iter().map(Tree::new).collect_vec()
    }

    fn mock_forest_from_heights(heights: Vec<Vec<u8>>) -> Forest {
        Forest { rows: heights.into_iter().map(mock_trees_from_heights).collect_vec() }
    }

    #[test]
    fn test_width() {
        let forest = mock_forest_from_heights(vec![vec![1, 2, 3]]);
        assert_eq!(forest.width(), 3);
    }

    #[test]
    fn test_height() {
        let forest = mock_forest_from_heights(vec![vec![1], vec![2], vec![3]]);
        assert_eq!(forest.height(), 3);
    }

    #[test]
    fn test_left_of() {
        let mut forest = mock_forest_from_heights(vec![vec![1, 2, 3]]);
        let mut left_iter = forest.left_of(0, 2).map(|tree| tree.height);
        assert_eq!(left_iter.next(), Some(2));
        assert_eq!(left_iter.next(), Some(1));
        assert_eq!(left_iter.next(), None);
    }

    #[test]
    fn test_right_of() {
        let mut forest = mock_forest_from_heights(vec![vec![1, 2, 3]]);
        let mut right_iter = forest.right_of(0, 0).map(|tree| tree.height);
        assert_eq!(right_iter.next(), Some(2));
        assert_eq!(right_iter.next(), Some(3));
        assert_eq!(right_iter.next(), None);
    }

    #[test]
    fn test_top_of() {
        let mut forest = mock_forest_from_heights(vec![vec![1], vec![2], vec![3]]);
        let mut top_iter = forest.top_of(2, 0).map(|tree| tree.height);
        assert_eq!(top_iter.next(), Some(2));
        assert_eq!(top_iter.next(), Some(1));
        assert_eq!(top_iter.next(), None);
    }

    #[test]
    fn test_bottom_of() {
        let mut forest = mock_forest_from_heights(vec![vec![1], vec![2], vec![3]]);
        let mut bottom_iter = forest.bottom_of(0, 0).map(|tree| tree.height);
        assert_eq!(bottom_iter.next(), Some(2));
        assert_eq!(bottom_iter.next(), Some(3));
        assert_eq!(bottom_iter.next(), None);
    }

    #[test]
    fn test_count_visible_trees() {
        let mut trees = mock_trees_from_heights(vec![3, 4]);
        assert_eq!(count_visible_trees_from(trees.iter_mut(), 2), 1);
        assert_eq!(count_visible_trees_from(trees.iter_mut(), 3), 1);
        assert_eq!(count_visible_trees_from(trees.iter_mut(), 4), 2);

        let mut trees = mock_trees_from_heights(vec![]);
        assert_eq!(count_visible_trees_from(trees.iter_mut(), 9), 0);
    }
}