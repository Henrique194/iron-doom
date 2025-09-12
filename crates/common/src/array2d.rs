use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Array2d<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Array2d<T> {
    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let spot = x + (y * self.width);
        self.data.get(spot)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let spot = x + (y * self.width);
        self.data.get_mut(spot)
    }

    #[inline]
    #[must_use]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: Default> Array2d<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, T::default);

        Self {
            data,
            width,
            height,
        }
    }
}

impl<T: Copy> Array2d<T> {
    pub fn copy_to_slice(&self, slice: &mut [T]) {
        slice.copy_from_slice(&self.data);
    }

    pub fn copy_from(&mut self, src: &Array2d<T>) {
        self.data.copy_from_slice(&src.data);
    }
}

impl<T: Clone> Array2d<T> {
    pub fn fill(&mut self, val: T) {
        self.data.fill(val);
    }
}

impl<T: Clone> Clone for Array2d<T> {
    fn clone(&self) -> Self {
        Array2d {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl<T> Index<(usize, usize)> for Array2d<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).unwrap()
    }
}
