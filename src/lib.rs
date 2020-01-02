use rand;
use generic_matrix;

use generic_matrix::Matrix;
use rand::Rng;
use std::cmp;
use std::ops::Range;
use std::fmt::{Debug, Formatter, Error, Write};
use crate::matrix::{MatrixExt, Cell, CellMut, MatrixExtPartEq};

mod matrix;

#[derive(PartialEq)]
enum FieldCell {
    Empty,
    NearMine(u8),
    Mine,
}
impl Debug for FieldCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let tmp;
        f.write_str(match &self {
            FieldCell::Empty => "_",
            FieldCell::NearMine(x) => { tmp = x.to_string(); &tmp },
            FieldCell::Mine => "!",
        })
    }
}
pub struct Field(Matrix<FieldCell>);
impl Field {
    pub fn generate(
        h: usize,
        w: usize,
        mines: usize,
        except_y: usize,
        except_x: usize,
        rand: &mut impl Rng,
    ) -> Field {
        assert!(mines < w * h);
        let mut field = Field(Matrix::from_fn(h, w, |_, _| FieldCell::Empty));
        let mut mines_set = 0;
        while mines_set < mines {
            if field.set_mine(rand.gen_range(0, h), rand.gen_range(0, w), except_y, except_x) {
                mines_set += 1;
            }
        }
        field
    }
    fn set_mine(&mut self, y: usize, x: usize, except_y: usize, except_x: usize) -> bool {
        if self.around_x(x).contains(&except_x) && self.around_y(y).contains(&except_y) {
            return false;
        }
        if let FieldCell::Mine = self.0[(y, x)] {
            return false;
        }
        self.0[(y, x)] = FieldCell::Mine;
        self.0.for_each_around_mut(y, x, true, |CellMut { value: cell, x: _, y: _ }| {
            *cell = match *cell {
                FieldCell::Empty => FieldCell::NearMine(1),
                FieldCell::NearMine(x) => FieldCell::NearMine(x + 1),
                FieldCell::Mine => FieldCell::Mine,
            };
        });
        true
    }
    fn around_y(&self, y: usize) -> Range<usize> {
        y.saturating_sub(1)..cmp::min(self.0.row(), y + 2)
    }
    fn around_x(&self, x: usize) -> Range<usize> {
        x.saturating_sub(1)..cmp::min(self.0.column(), x + 2)
    }
}
impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for j in 0..self.0.row() {
            for i in 0..self.0.column() {
                self.0[(j, i)].fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Result::Ok(())
    }
}

#[derive(PartialEq, Clone, Copy)]
enum CellView {
    Hidden,
    Flagged,
    Shown,
}
pub struct Session<'a> {
    field: &'a Field,
    presentation: Matrix<CellView>,
}
impl<'a> Session<'a> {
    pub fn from_field(field: &'a Field, except_y: usize, except_x: usize) -> Session<'a> {
        let mut sess = Session {
            field: &field,
            presentation: Matrix::from_fn(field.0.row(), field.0.column(), |_, _| CellView::Hidden),
        };
        sess.reveal_at(except_y, except_x);
        sess
    }
    fn reveal_at(&mut self, y: usize, x: usize) {
        self.presentation[(y, x)] = CellView::Shown;
        self.field.0.for_each_around(y, x, true, | Cell { value: cell, y, x } | {
            if *cell != FieldCell::Mine {
                // Empty || NearMine which is not shown yet
                if self.presentation[(y, x)] != CellView::Shown {
                    self.presentation[(y, x)] = CellView::Shown;
                    if let FieldCell::Empty = *cell { // and reveal more
                        self.reveal_at(y, x);
                    }
                }
            }
        })
    }

    pub fn auto_flag(&mut self) -> usize {
        let mut flagged: usize = 0;
        for y in 0..self.presentation.row() {
            for x in 0..self.presentation.column() {
                if let CellView::Shown = self.presentation[(y, x)] {
                    if let FieldCell::NearMine(mines) = self.field.0[(y, x)] {
                        let potentially_mines = self.presentation.count_around_not(
                            y, x, true, CellView::Shown // Hidden || Flagged
                        );
                        if potentially_mines == mines {
                            flagged += self.presentation.replace_around(
                                y, x, true, CellView::Hidden, CellView::Flagged
                            ) as usize;
                        }
                    }
                }
            }
        }
        flagged
    }

    pub fn auto_open(&mut self) -> usize {
        let mut opened: usize = 0;
        for y in 0..self.presentation.row() {
            for x in 0..self.presentation.column() {
                if let CellView::Shown = self.presentation[(y, x)] {
                    if let FieldCell::NearMine(mines) = self.field.0[(y, x)] {
                        let flagged = self.presentation.count_around_of(
                            y, x, true, CellView::Flagged
                        );
                        if flagged == mines {
                            for &coords in self.presentation.coordinates_around(y, x, true).iter() {
                                if let Some(coords) = coords {
                                    if self.presentation[coords] == CellView::Hidden {
                                        self.reveal_at(coords.0, coords.1);
                                        opened += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        opened
    }
}
impl Debug for Session<'_> {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for j in 0..self.presentation.row() {
            for i in 0..self.presentation.column() {
                match self.presentation[(j, i)] {
                    CellView::Hidden => f.write_char('█')?,
                    CellView::Flagged => f.write_char('¶')?,
                    CellView::Shown => self.field.0[(j, i)].fmt(f)?,
                };
            }
            f.write_char('\n')?;
        }
        Result::Ok(())
    }
}
