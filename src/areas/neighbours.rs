/// A definition of neighbouring elements, used in various algorithms
/// throughout the crate.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BidiNeighbours {
    /// Consider only the four fully adjacent elements as neighbours,
    /// that is the element directly above, directly to the right,
    /// directly below an directly to the left.
    Adjacent,
    /// Consider the entire 8 bordering elements as neighbours:
    /// all the 4 elements of [`BidiNeighbours::Adjacent`] and
    /// he 4 elements that touch only with a corner, that is the
    /// element above to the right, below to the right, below to
    /// the left and above to the left.
    Bordering,
}

impl BidiNeighbours {
    pub(crate) fn generate_points_on(
        self,
        v: &mut Vec<(usize, usize)>,
        pos: (usize, usize),
        width: usize,
        height: usize,
    ) {
        let (x, y) = (pos.0, pos.1);
        let go_n = y > 0;
        let go_w = x > 0;
        let go_e = x < width - 1;
        let go_s = y < height - 1;

        match self {
            BidiNeighbours::Adjacent => {
                // West
                if go_w {
                    v.push((x - 1, y));
                }
                // South
                if go_s {
                    v.push((x, y + 1));
                }
                // East
                if go_e {
                    v.push((x + 1, y));
                }
                // North
                if go_n {
                    v.push((x, y - 1));
                }
            }
            BidiNeighbours::Bordering => {
                // North-West
                if go_n && go_w {
                    v.push((x - 1, y - 1));
                }
                // West
                if go_w {
                    v.push((x - 1, y));
                }
                // South-West
                if go_s && go_w {
                    v.push((x - 1, y + 1));
                }
                // South
                if go_s {
                    v.push((x, y + 1));
                }
                // South-East
                if go_s && go_e {
                    v.push((x + 1, y + 1));
                }
                // East
                if go_e {
                    v.push((x + 1, y));
                }
                // North-East
                if go_n && go_e {
                    v.push((x + 1, y - 1));
                }
                // North
                if go_n {
                    v.push((x, y - 1));
                }
            }
        }
    }

    pub(crate) fn prealloc_vec(self) -> Vec<(usize, usize)> {
        match self {
            BidiNeighbours::Adjacent => Vec::with_capacity(4),
            BidiNeighbours::Bordering => Vec::with_capacity(8),
        }
    }
}
