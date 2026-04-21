pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub contents: Vec<T>,
}

impl<T : Default + Copy> Grid<T> {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            contents: vec![T::default(); width*height]
        }
    }
}

impl<T> Grid<T> {
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        return x < self.width && y < self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if !self.in_bounds(x, y) {
            None
        } else {
            Some(self.get_unchecked(x, y))
        }
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
        return &self.contents[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
        if !self.in_bounds(x, y) {
            panic!("Index ({x} {y}) out of bounds for grid of size ({} {})", self.width, self.height);
        }
        self.contents[y*self.width + x] = value;
        true
    }

    pub fn direct_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize, &T)> {
        let mut neighbours = Vec::new();
        if self.in_bounds(x, y) {
            if y > 0 {
                neighbours.push((x, y - 1, self.get_unchecked(x, y - 1)));
            }
            if x < self.width - 1 {
                neighbours.push((x + 1, y, self.get_unchecked(x + 1, y)));
            }
            if y < self.height - 1 {
                neighbours.push((x, y + 1, self.get_unchecked(x, y + 1)));
            }
            if x > 0 {
                neighbours.push((x - 1, y, self.get_unchecked(x - 1, y)));
            }
        }
        neighbours
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.contents.iter()
    }

    pub fn row(&self, y: usize) -> Option<&[T]> {
        if y >= self.width {
            None
        } else {
            Some(&self.contents[y*self.width..(y+1)*self.width])
        }
    }
}