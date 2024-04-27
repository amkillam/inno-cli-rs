// type
//   DynamicArray = record
//     data: ppointer;  // pointer to array of void pointers
//     cur_size: cuint; // currently allocated size of the array
//   end;

use std::alloc::realloc;
use std::ops::Index;

#[repr(C)]
pub struct PascalDynamicArray<T: Sized> {
    data: *mut *mut T, //Nested pointer to first element of the array
    cur_size: u32,
}

impl<T> PascalDynamicArray<T> {
    pub fn new() -> Self {
        Self {
            data: std::ptr::null_mut(),
            cur_size: 0,
        }
    }

    pub fn append(&mut self, item: *mut T) {
        self.cur_size += 1;

        let layout = std::alloc::Layout::new::<*mut T>();
        self.data = unsafe { realloc((*self.data) as *mut u8, layout, self.cur_size as usize) }
            as *mut *mut T;
        let item_ptr = self.data.wrapping_add(self.cur_size as usize);
        unsafe {
            *item_ptr = item;
        }
    }

    pub fn get(&self, index: usize) -> *mut T {
        unsafe { *(self.data.wrapping_add(index)) }
    }

    pub fn len(&self) -> u32 {
        self.cur_size
    }

    pub fn clear(&mut self) {
        self.cur_size = 0;
        self.data = std::ptr::null_mut();
    }

    pub fn free(&mut self) {
        unsafe {
            for i in 0..self.cur_size {
                //Takes ownership of the boxed value, deallocating it automatically on return
                let _ = Box::from_raw(self.data.wrapping_add(i as usize));
            }
            let _ = Box::from_raw(self.data);
        }
    }
}

impl<T> Drop for PascalDynamicArray<T> {
    fn drop(&mut self) {
        self.free();
    }
}

impl<T> Index<usize> for PascalDynamicArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*(self.get(index)) }
    }
}

impl<T> TryFrom<*mut *mut T> for PascalDynamicArray<T> {
    type Error = ();

    fn try_from(value: *mut *mut T) -> Result<Self, Self::Error> {
        let mut cur_size: u32 = 0;
        while !value.wrapping_add(cur_size as usize).is_null() {
            cur_size += 1;
        }
        Ok(Self {
            data: value,
            cur_size,
        })
    }
}

impl<T> TryInto<*mut *mut T> for PascalDynamicArray<T> {
    type Error = ();

    fn try_into(self) -> Result<*mut *mut T, Self::Error> {
        Ok(self.data)
    }
}

impl<T> TryFrom<Vec<*mut T>> for PascalDynamicArray<T> {
    type Error = ();

    fn try_from(mut value: Vec<*mut T>) -> Result<Self, Self::Error> {
        let mut cur_size: u32 = 0;
        while !value[cur_size as usize].is_null() {
            cur_size += 1;
        }
        Ok(Self {
            data: value.as_mut_ptr(),
            cur_size,
        })
    }
}

impl<T: Copy> TryInto<Vec<T>> for PascalDynamicArray<T> {
    type Error = ();

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        let mut vec = Vec::with_capacity(self.cur_size as usize);
        for i in 0..self.cur_size {
            vec.push(self[i as usize]);
        }
        Ok(vec)
    }
}

impl<T> TryFrom<&mut [T]> for PascalDynamicArray<T> {
    type Error = ();

    fn try_from(value: &mut [T]) -> Result<Self, Self::Error> {
        let cur_size = value.len() as u32;
        Ok(Self {
            data: value.as_mut_ptr().cast(),
            cur_size,
        })
    }
}

impl<T: 'static> TryInto<&[T]> for PascalDynamicArray<T> {
    type Error = ();

    fn try_into(self) -> Result<&'static [T], Self::Error> {
        let slice = unsafe { std::slice::from_raw_parts(*self.data, self.cur_size as usize) };
        Ok(slice)
    }
}

//Unit test:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_dynamic_array() {
        let mut array = PascalDynamicArray::new();
        let mut vec = Vec::new();
        for i in 0..10 {
            let item = Box::into_raw(Box::new(i));
            array.append(item);
            vec.push(i);
        }
        assert_eq!(array.len(), 10);
        for i in 0..10 {
            assert_eq!(unsafe { *(array.get(i)) }, vec[i]);
        }
        let vec2: Vec<i32> = array.try_into().unwrap();
        assert_eq!(vec, vec2);
    }

    #[test]
    fn test_pascal_dynamic_array_clear() {
        let mut array = PascalDynamicArray::new();
        for i in 0..10 {
            let item = Box::into_raw(Box::new(i));
            array.append(item);
        }
        array.clear();
        assert_eq!(array.len(), 0);
    }
}
