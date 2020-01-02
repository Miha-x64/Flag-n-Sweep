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

    fn coordinates_around(&self, y: usize, x: usize, corners: bool) -> [Option<(usize, usize)>; 8];

    fn for_each_around(&self, y: usize, x: usize, corners: bool, f: impl FnMut(Cell<'_, T>));
    fn for_each_around_mut(&mut self, y: usize, x: usize, corners: bool, f: impl FnMut(CellMut<'_, T>));

    fn count_around_by(&self, y: usize, x: usize, corners: bool, predicate: impl FnMut(Cell<'_, T>) -> bool) -> u8;
}
pub trait MatrixExtPartEq<T: PartialEq + Copy> {
    fn count_around_of(&self, y: usize, x: usize, corners: bool, needle: T) -> u8;
    fn count_around_not(&self, y: usize, x: usize, corners: bool, needle: T) -> u8;
    fn replace_around(&mut self, y: usize, x: usize, corners: bool, what: T, with: T) -> u8;
}

impl<'a, T: 'a> MatrixExt<'a, T> for Matrix<T> {

    fn cell_at(&'a self, y: usize, x: usize) -> Cell<'a, T> {
        Cell { value: &self[(y, x)], y, x, }
    }

    fn cell_at_mut(&'a mut self, y: usize, x: usize) -> CellMut<'a, T> {
        CellMut { value: &mut self[(y, x)], y, x, }
    }

    fn coordinates_around(&self, y: usize, x: usize, corners: bool) -> [Option<(usize, usize)>; 8] {
        let (c, has_l, has_t, has_r, has_b) =
            (corners, x != 0, y != 0, x < self.column() - 1, y < self.row()-1);

        [
            if c && has_t && has_l { Some((y - 1, x - 1)) } else { None },
            if      has_t          { Some((y - 1, x))     } else { None },
            if c && has_t && has_r { Some((y - 1, x + 1)) } else { None },
            if      has_l          { Some((y, x - 1))     } else { None },
            // 3.5: central cell
            if      has_r          { Some((y, x + 1))     } else { None },
            if c && has_b && has_l { Some((y + 1, x - 1)) } else { None },
            if      has_b          { Some((y + 1, x))     } else { None },
            if c && has_b && has_r { Some((y + 1, x + 1)) } else { None },
        ]
    }

    fn for_each_around(&self, y: usize, x: usize, corners: bool, mut f: impl FnMut(Cell<'_, T>)) {
        for &coords in self.coordinates_around(y, x, corners).iter() {
            if let Some((y, x)) = coords {
                f(self.cell_at(y, x));
            }
        };
    }

    fn for_each_around_mut(&mut self, y: usize, x: usize, corners: bool, mut f: impl FnMut(CellMut<'_, T>)) {
        for &coords in self.coordinates_around(y, x, corners).iter() {
            if let Some((y, x)) = coords {
                f(self.cell_at_mut(y, x));
            }
        };
    }

    fn count_around_by(&self, y: usize, x: usize, corners: bool, mut predicate: impl FnMut(Cell<'_, T>) -> bool) -> u8 {
        let mut count: u8 = 0;
        self.for_each_around(
            y, x, corners,
            | cell | { if predicate(cell) { count += 1; } }
        );
        count
    }
}

impl<T: PartialEq + Copy> MatrixExtPartEq<T> for Matrix<T> {

    fn count_around_of(&self, y: usize, x: usize, corners: bool, needle: T) -> u8 {
        self.count_around_by(y, x, corners, | Cell { value: &value, y: _, x: _ } | {
            value == needle
        })
    }

    fn count_around_not(&self, y: usize, x: usize, corners: bool, needle: T) -> u8 {
        self.count_around_by(y, x, corners, | Cell { value: &value, y: _, x: _ } | {
            value != needle
        })
    }

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
