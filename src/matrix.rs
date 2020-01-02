use generic_matrix;

use generic_matrix::Matrix;


pub struct Cell<'a, T> {
    pub value: &'a T,
    pub y: usize,
    pub x: usize,
}

pub struct CellMut<'a, T> {
    pub value: &'a mut T,
    pub y: usize,
    pub x: usize,
}

pub trait MatrixExt<'a, T: 'a> {

    fn cell_at(&'a self, y: usize, x: usize) -> Cell<'a, T>;
    fn cell_at_mut(&'a mut self, y: usize, x: usize) -> CellMut<'a, T>;

    fn for_each_around(&self, y: usize, x: usize, corners: bool, f: impl FnMut(Cell<'_, T>));
    fn for_each_around_mut(&mut self, y: usize, x: usize, corners: bool, f: impl FnMut(CellMut<'_, T>));

    fn count_around(&self, y: usize, x: usize, corners: bool, predicate: fn(Cell<'_, T>) -> bool) -> u8;
}
pub trait MatrixExtPartEq<T: PartialEq + Copy> {
    fn replace_around(&mut self, y: usize, x: usize, corners: bool, what: T, with: T) -> u8;
}

impl<'a, T: 'a> MatrixExt<'a, T> for Matrix<T> {

    fn cell_at(&'a self, y: usize, x: usize) -> Cell<'a, T> {
        Cell { value: &self[(y, x)], y, x, }
    }

    fn cell_at_mut(&'a mut self, y: usize, x: usize) -> CellMut<'a, T> {
        CellMut { value: &mut self[(y, x)], y, x, }
    }

    fn for_each_around(&self, y: usize, x: usize, corners: bool, mut f: impl FnMut(Cell<'_, T>)) {
        let (c, has_l, has_t, has_r, has_b) =
            (corners, x != 0, y != 0, x < self.column() - 1, y < self.row()-1);

        if c && has_t && has_l { f(self.cell_at(y - 1, x - 1)); };
        if      has_t          { f(self.cell_at(y - 1, x));     };
        if c && has_t && has_r { f(self.cell_at(y - 1, x + 1)); };
        if      has_l          { f(self.cell_at(y, x - 1));     };
        // 3.5: central cell
        if      has_r          { f(self.cell_at(y, x + 1));     };
        if c && has_b && has_l { f(self.cell_at(y + 1, x - 1)); };
        if      has_b          { f(self.cell_at(y + 1, x));     };
        if c && has_b && has_r { f(self.cell_at(y + 1, x + 1)); };
    }

    fn for_each_around_mut(&mut self, y: usize, x: usize, corners: bool, mut f: impl FnMut(CellMut<'_, T>)) {
        let (c, has_l, has_t, has_r, has_b) =
            (corners, x != 0, y != 0, x < self.column() - 1, y < self.row()-1);

        if c && has_t && has_l { f(self.cell_at_mut(y - 1, x - 1)); };
        if      has_t          { f(self.cell_at_mut(y - 1, x));     };
        if c && has_t && has_r { f(self.cell_at_mut(y - 1, x + 1)); };
        if      has_l          { f(self.cell_at_mut(y, x - 1));     };
        // 3.5: central cell
        if      has_r          { f(self.cell_at_mut(y, x + 1));     };
        if c && has_b && has_l { f(self.cell_at_mut(y + 1, x - 1)); };
        if      has_b          { f(self.cell_at_mut(y + 1, x));     };
        if c && has_b && has_r { f(self.cell_at_mut(y + 1, x + 1)); };
    }

    fn count_around(&self, y: usize, x: usize, corners: bool, predicate: fn(Cell<'_, T>) -> bool) -> u8 {
        let mut count: u8 = 0;
        self.for_each_around(
            y, x, corners,
            | cell | { if predicate(cell) { count += 1; } }
        );
        count
    }
}

impl<T: PartialEq + Copy> MatrixExtPartEq<T> for Matrix<T> {
    fn replace_around(&mut self, y: usize, x: usize, corners: bool, what: T, with: T) -> u8 {
        let mut replaced: u8 = 0;
        self.for_each_around_mut(y, x, corners, | CellMut { value, y: _, x: _ } | {
            if *value == what {
                *value = with;
                replaced += 1;
            }
        });
        replaced
    }
}
