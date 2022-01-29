use crate::BidiRect;

pub(crate) enum OnRectState {
    NotStarted,
    Iterating(usize, usize),
    Terminated,
}

impl OnRectState {
    pub(crate) fn assert_not_started(&self, caller: &str) {
        match self {
            OnRectState::NotStarted => (),
            _ => panic!("Can't call '{}' after enumeration has started.", caller),
        }
    }

    pub(crate) fn advance(&mut self, rect: &BidiRect, by_col: bool) {
        *self = match self {
            OnRectState::Terminated => OnRectState::Terminated,
            OnRectState::NotStarted => {
                if rect.contains(rect.x, rect.y) {
                    OnRectState::Iterating(rect.x, rect.y)
                } else {
                    OnRectState::Terminated
                }
            }
            OnRectState::Iterating(mut x, mut y) if by_col => {
                y += 1;
                if y >= rect.max_y() {
                    x += 1;
                    y = rect.y;
                }
                if rect.contains(x, y) {
                    OnRectState::Iterating(x, y)
                } else {
                    OnRectState::Terminated
                }
            }
            OnRectState::Iterating(mut x, mut y) => {
                x += 1;
                if x >= rect.max_x() {
                    y += 1;
                    x = rect.x;
                }
                if rect.contains(x, y) {
                    OnRectState::Iterating(x, y)
                } else {
                    OnRectState::Terminated
                }
            }
        };
    }
}
