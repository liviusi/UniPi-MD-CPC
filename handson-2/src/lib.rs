#[derive(Debug)]
pub struct SegmentTreeFirstExercise {
    keys: Vec<(i32, usize, usize)>,
    lazy: Vec<i32>,
    len: usize,
}

impl SegmentTreeFirstExercise {
    pub fn from_vector(v: Vec<i32>) -> Self {
        let len = v.len();
        let size = if len.count_ones() == 1 {
            (len << 1) - 1
        } else {
            ((1 << (64 - len.leading_zeros())) << 1) - 1
        };
        let mut keys = vec![(0_i32, 0, 0); size];
        let mut lazy = vec![0_i32; size];
        Self::rec_fill_tree_from_vector(&v, &mut keys, &mut lazy, 0, len - 1, 0);

        Self { keys, lazy, len }
    }

    fn rec_fill_tree_from_vector(
        v: &Vec<i32>,
        keys: &mut Vec<(i32, usize, usize)>,
        lazy: &mut Vec<i32>,
        left: usize,
        right: usize,
        current_position: usize,
    ) {
        if right < left {
            return;
        }
        if right == left {
            keys[current_position] = (v[left], left, right);
            lazy[current_position] = keys[current_position].0;
            return;
        }
        let mid: usize = (left + right) / 2;
        Self::rec_fill_tree_from_vector(v, keys, lazy, left, mid, (current_position << 1) + 1);
        Self::rec_fill_tree_from_vector(v, keys, lazy, mid + 1, right, (current_position << 1) + 2);

        keys[current_position] = (
            std::cmp::max(
                keys[(current_position << 1) + 1].0,
                keys[(current_position << 1) + 2].0,
            ),
            left,
            right,
        );
        lazy[current_position] = keys[current_position].0;
    }

    pub fn update(&mut self, i: usize, j: usize, t: i32) {
        if i > self.len || i > j {
            return;
        }
        self.rec_update(0, i - 1, j - 1, t);
    }

    fn rec_update(&mut self, current_index: usize, left: usize, right: usize, t: i32) {
        if self.lazy[current_index] != self.keys[current_index].0 {
            // if there's something to propagate, apply the update to the current node
            self.keys[current_index].0 =
                std::cmp::min(self.keys[current_index].0, self.lazy[current_index]);

            if self.keys[current_index].1 != self.keys[current_index].2 {
                // if current node is not a leaf, update children
                self.lazy[(current_index << 1) + 1] = std::cmp::min(
                    self.lazy[current_index],
                    self.lazy[(current_index << 1) + 1],
                );
                self.lazy[(current_index << 1) + 2] = std::cmp::min(
                    self.lazy[current_index],
                    self.lazy[(current_index << 1) + 2],
                );
            }

            // value updated, reset lazy for current node
            self.lazy[current_index] = self.keys[current_index].0;
        }

        // no overlap
        if self.keys[current_index].2 < left || right < self.keys[current_index].1 {
            return;
        }

        // total overlap
        if left <= self.keys[current_index].1 && self.keys[current_index].2 <= right {
            self.keys[current_index].0 = std::cmp::min(self.keys[current_index].0, t);
            if self.keys[current_index].1 != self.keys[current_index].2 {
                // if current node is not a leaf, update children
                self.lazy[(current_index << 1) + 1] =
                    std::cmp::min(t, self.lazy[(current_index << 1) + 1]);

                self.lazy[(current_index << 1) + 2] =
                    std::cmp::min(t, self.lazy[(current_index << 1) + 2]);
            }
            return;
        }

        // partial overlap
        self.rec_update((current_index << 1) + 1, left, right, t);
        self.rec_update((current_index << 1) + 2, left, right, t);

        // update current node with max of its updated children
        self.keys[current_index].0 = std::cmp::max(
            self.keys[(current_index << 1) + 1].0,
            self.keys[(current_index << 1) + 2].0,
        );
        self.lazy[current_index] = self.keys[current_index].0;
    }

    pub fn max(&mut self, i: usize, j: usize) -> i32 {
        self.rec_max(0, i - 1, j - 1)
    }

    fn rec_max(&mut self, current_index: usize, left: usize, right: usize) -> i32 {
        // no overlap
        if self.keys[current_index].2 < left || right < self.keys[current_index].1 {
            return i32::MIN;
        }
        if self.lazy[current_index] != self.keys[current_index].0 {
            // if there's something to propagate, apply the update to the current node
            self.keys[current_index].0 =
                std::cmp::min(self.keys[current_index].0, self.lazy[current_index]);

            if self.keys[current_index].1 != self.keys[current_index].2 {
                // if current node is not a leaf, update children
                self.lazy[(current_index << 1) + 1] = std::cmp::min(
                    self.lazy[current_index],
                    self.lazy[(current_index << 1) + 1],
                );
                self.lazy[(current_index << 1) + 2] = std::cmp::min(
                    self.lazy[current_index],
                    self.lazy[(current_index << 1) + 2],
                );
            }

            // value updated, reset lazy for current node
            self.lazy[current_index] = self.keys[current_index].0;
        }

        // total overlap
        if left <= self.keys[current_index].1 && self.keys[current_index].2 <= right {
            return self.keys[current_index].0;
        }

        // partial overlap
        let max_left = self.rec_max((current_index << 1) + 1, left, right);
        let max_right = self.rec_max((current_index << 1) + 2, left, right);
        std::cmp::max(max_left, max_right)
    }

    pub fn print_segtree(&self) {
        println!("Keys: {:?}\nLazy:{:?}", self.keys, self.lazy);
    }
}

pub struct SegmentTreeSecondExercise {
    keys: Vec<u32>,
    lazy: Vec<u32>,
    len: usize,
}

impl SegmentTreeSecondExercise {
    pub fn from_segment_vector(segments: Vec<(usize, usize)>) -> Self {
        let len = segments.len();
        let size = if len.count_ones() == 1 {
            (len << 1) - 1
        } else {
            ((1 << (64 - len.leading_zeros())) << 1) - 1
        };
        let mut keys = vec![0_u32; size];
        let mut lazy = vec![0_u32; size];

        for segment in segments {
            Self::insert_segment(segment, &mut keys, &mut lazy, 0, len - 1, 0);
        }

        Self { keys, lazy, len }
    }

    fn insert_segment(
        segment: (usize, usize),
        keys: &mut Vec<u32>,
        lazy: &mut Vec<u32>,
        left: usize,
        right: usize,
        current_index: usize,
    ) {
        if lazy[current_index] != 0 {
            // there are updates to propagate
            keys[current_index] += lazy[current_index];

            // propagate update to its children
            if left != right {
                lazy[(current_index << 1) + 1] += lazy[current_index];
                lazy[(current_index << 1) + 2] += lazy[current_index];
            }

            // value updated, reset lazy
            lazy[current_index] = 0;
        }

        // no overlap
        if segment.0 > right || segment.1 < left {
            return;
        }

        // total overlap
        if left >= segment.0 && right <= segment.1 {
            keys[current_index] += 1;

            if left != right {
                lazy[(current_index << 1) + 1] += 1;
                lazy[(current_index << 1) + 2] += 1;
            }

            return;
        }

        // partial overlap
        let mid = left + (right - left) / 2;
        Self::insert_segment(segment, keys, lazy, left, mid, (current_index << 1) + 1);
        Self::insert_segment(
            segment,
            keys,
            lazy,
            mid + 1,
            right,
            (current_index << 1) + 2,
        );

        keys[current_index] = std::cmp::max(
            keys[(current_index << 1) + 1],
            keys[(current_index << 1) + 2],
        );
    }

    pub fn is_there(&mut self, i: usize, j: usize, k: u32) -> i32 {
        if self.rec_is_there(i, j, k, 0, self.len - 1, 0) {
            1
        } else {
            0
        }
    }

    fn rec_is_there(
        &mut self,
        query_left_index: usize,
        query_right_index: usize,
        k: u32,
        left: usize,
        right: usize,
        current_index: usize,
    ) -> bool {
        if self.lazy[current_index] != 0 {
            // there are updates to propagate
            self.keys[current_index] += self.lazy[current_index];

            // propagate update to its children
            if left != right {
                self.lazy[(current_index << 1) + 1] += self.lazy[current_index];
                self.lazy[(current_index << 1) + 2] += self.lazy[current_index];
            }

            // value updated, reset lazy
            self.lazy[current_index] = 0;
        }

        // no overlap
        if self.keys[current_index] < k {
            return false;
        }
        if query_left_index > right || query_right_index < left {
            return false;
        }

        // total overlap
        if left >= query_left_index && right <= query_right_index {
            if self.keys[current_index] == k {
                return true;
            }
            if left == right {
                return false;
            }
        }

        // no overlap
        let mid = left + (right - left) / 2;
        self.rec_is_there(
            query_left_index,
            query_right_index,
            k,
            left,
            mid,
            (current_index << 1) + 1,
        ) || self.rec_is_there(
            query_left_index,
            query_right_index,
            k,
            mid + 1,
            right,
            (current_index << 1) + 2,
        )
    }

    pub fn print_segtree(&self) {
        println!("Keys: {:?}\nLazy:{:?}", self.keys, self.lazy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let mut segtree = SegmentTreeFirstExercise::from_vector(vec![5, 1, 4, 3, 2]);
        segtree.print_segtree();

        segtree.update(1, 2, 2);
        assert_eq!(segtree.max(2, 4), 4);
        assert_eq!(segtree.max(1, 2), 2);
    }

    #[test]
    fn test_from_segments() {
        let mut segtree = SegmentTreeSecondExercise::from_segment_vector(vec![
            (0, 4),
            (1, 3),
            (1, 2),
            (1, 1),
            (0, 0),
        ]);

        segtree.print_segtree();
        assert_eq!(segtree.is_there(0, 4, 4), 1);
        assert_eq!(segtree.is_there(0, 4, 0), 0);
        assert_eq!(segtree.is_there(1, 3, 1), 0);
        assert_eq!(segtree.is_there(1, 4, 1), 1);
    }
}
