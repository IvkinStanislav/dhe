pub struct Allocator {
    /// (mID, size)
    data: Vec<(i32, i32)>,
}

impl Allocator {
    pub fn new(n: i32) -> Self {
        Self { data: vec![(0, n)] }
    }

    pub fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let mut insert_index = None;
        let mut target_free_size = 0;
        let mut point = 0;

        for (index, (exist_m_id, exist_size)) in self.data.iter().enumerate() {
            if *exist_m_id == 0 && size <= *exist_size {
                insert_index = Some(index);
                target_free_size = exist_size - size;
                break;
            }
            point += exist_size;
        }

        if let Some(index) = insert_index {
            self.data.remove(index);
            if target_free_size > 0 {
                self.data.insert(index, (0, target_free_size));
            }
            self.data.insert(index, (m_id, size));
            self.normalize();
            point
        } else {
            -1
        }
    }

    pub fn free(&mut self, m_id: i32) -> i32 {
        let mut size = 0;
        for (exist_m_id, exist_size) in self.data.iter_mut() {
            if *exist_m_id == m_id {
                *exist_m_id = 0;
                size += *exist_size;
            }
        }

        self.normalize();
        size
    }

    fn normalize(&mut self) {
        let mut i = 1;
        while i < self.data.len() {
            if self.data[i - 1].0 == self.data[i].0 {
                self.data[i].1 += self.data[i - 1].1;
                self.data.remove(i - 1);
            }
            i += 1;
        }
    }
}
