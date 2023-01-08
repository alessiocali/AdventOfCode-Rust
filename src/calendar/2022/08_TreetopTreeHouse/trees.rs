use enumset::{ EnumSet, EnumSetType };
use std::{ iter::{ Rev, Skip } };

#[derive(EnumSetType)]
pub enum TreeVisibility {
    North,
    South,
    West,
    East
}

pub struct Tree {
    pub height: u8,
    pub visibility: EnumSet<TreeVisibility>,
    pub scenic_score: u32
}

impl Tree {
    pub fn new(height: u8) -> Tree {
        Tree { height, visibility: EnumSet::<TreeVisibility>::all(), scenic_score: 0 }
    }
}

pub struct Forest {
    pub rows: Vec<Vec<Tree>>
}

impl Forest {
    // General note: requiring row/column iterators to borrow mutable is suboptimal,
    // as sometimes we want just to read borrow immutably. However AFAIK Rust does not
    // support templating on mutability so I'd need to write the same iterators twice,
    // which I'd rather not.

    pub fn iter_row(&mut self, row: usize) -> RowIterator<'_> {
        RowIterator { row_iter: self.rows.get_mut(row).unwrap().iter_mut() }
    }

    pub fn iter_col(&mut self, col: usize) -> ColumnIterator<'_> {
        ColumnIterator { column_iter: self.rows.iter_mut(), column_idx: col }
    }

    pub fn left_of<'a>(&'a mut self, row: usize, col: usize) -> Skip<Rev<RowIterator<'a>>> {
        let column_from_right = self.width() - col - 1;
        self.iter_row(row).rev().skip(column_from_right + 1)
    }

    pub fn right_of<'a>(&'a mut self, row: usize, col: usize) -> Skip<RowIterator<'a>> {
        self.iter_row(row).skip(col + 1)
    }

    pub fn top_of<'a>(&'a mut self, row: usize, col: usize) -> Skip<Rev<ColumnIterator<'a>>> {
        let row_from_bottom = self.height() - row - 1;
        self.iter_col(col).rev().skip(row_from_bottom + 1)
    }

    pub fn bottom_of<'a>(&'a mut self, row: usize, col: usize) -> Skip<ColumnIterator<'a>> {
        self.iter_col(col).skip(row + 1)
    }

    pub fn width(&self) -> usize {
        self.rows.get(0).map(|row| row.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }
}

pub struct RowIterator<'a> {
    row_iter: std::slice::IterMut<'a, Tree>
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = &'a mut Tree;

    fn next(&mut self) -> Option<Self::Item> {
        self.row_iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.row_iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for RowIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.row_iter.next_back()
    }
}

impl<'a> ExactSizeIterator for RowIterator<'a> {}

pub struct ColumnIterator<'a> {
    column_iter: std::slice::IterMut<'a, Vec<Tree>>,
    column_idx: usize
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = &'a mut Tree;

    fn next(&mut self) -> Option<Self::Item> {
        self.column_iter.next()?.iter_mut().skip(self.column_idx).next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.column_iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for ColumnIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.column_iter.next_back()?.iter_mut().skip(self.column_idx).next()
    }
}

impl<'a> ExactSizeIterator for ColumnIterator<'a> { }