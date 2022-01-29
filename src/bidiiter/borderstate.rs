use crate::BidiRect;
use crate::BidiRectSigned;

#[derive(Debug)]
pub(crate) enum IterBorderState {
    NotStarted,
    Iterating(isize, isize),
    Terminated,
}

impl IterBorderState {
    pub(crate) fn assert_not_started(&self, caller: &str) {
        match self {
            IterBorderState::NotStarted => (),
            _ => panic!("Can't call '{}' after enumeration has started.", caller),
        }
    }

    pub(crate) fn advance(&mut self, clip: &BidiRect, border: &BidiRectSigned) {
        loop {
            *self = match (&self, border.width, border.height) {
                (IterBorderState::Terminated, ..) => IterBorderState::Terminated,
                (IterBorderState::NotStarted, 0, 0) => IterBorderState::Terminated,
                (IterBorderState::NotStarted, ..) => IterBorderState::Iterating(border.x, border.y),
                (IterBorderState::Iterating(x, y), 1, _) => {
                    if *y < (border.max_y() - 1) {
                        IterBorderState::Iterating(*x, *y + 1)
                    } else {
                        IterBorderState::Terminated
                    }
                }
                (IterBorderState::Iterating(x, y), _, 1) => {
                    if *x < (border.max_x() - 1) {
                        IterBorderState::Iterating(*x + 1, *y)
                    } else {
                        IterBorderState::Terminated
                    }
                }
                (IterBorderState::Iterating(x, y), ..) => {
                    let (x, y) = (*x, *y);
                    if y == border.y + 1 && x == border.x {
                        IterBorderState::Terminated
                    } else if y == border.y && (x < border.max_x() - 1) {
                        IterBorderState::Iterating(x + 1, y)
                    } else if (x == border.max_x() - 1) && (y < border.max_y() - 1) {
                        IterBorderState::Iterating(x, y + 1)
                    } else if (y == border.max_y() - 1) && x > border.x {
                        IterBorderState::Iterating(x - 1, y)
                    } else if x == border.x && y > border.y {
                        IterBorderState::Iterating(x, y - 1)
                    } else {
                        unreachable!()
                    }
                }
            };

            match self {
                IterBorderState::Terminated => break,
                IterBorderState::NotStarted => unreachable!(),
                IterBorderState::Iterating(x, y) => {
                    if clip.contains_signed(*x, *y) && border.contains(*x, *y) {
                        break;
                    }
                }
            }
        }
    }
}
