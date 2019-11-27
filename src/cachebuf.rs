#[derive(Debug)]
pub struct CapacityError;

#[derive(Clone)]
#[repr(C, align(64))]
pub(crate) struct CacheBuf {
    pub(crate) len: u8,
    pub(crate) buf: [u8; 63],
}

impl CacheBuf {
    const CACHEBUF_LEN: u8 = 63;

    #[inline(always)]
    pub(crate) const fn new() -> Self {
        Self {
            len: Self::CACHEBUF_LEN,
            buf: [0; Self::CACHEBUF_LEN as usize],
        }
    }

    #[inline(always)]
    pub(crate) const fn len(&self) -> usize {
        (Self::CACHEBUF_LEN - self.len) as usize
    }

    #[inline(always)]
    pub(crate) const fn capacity(&self) -> usize {
        Self::CACHEBUF_LEN as usize
    }

    #[inline]
    pub(crate) const fn remaining_capacity(&self) -> usize {
        self.capacity() - self.len()
    }

    #[inline(always)]
    pub(crate) const fn is_full(&self) -> bool {
        self.len == 0
    }

    #[inline(always)]
    pub(crate) const fn is_empty(&self) -> bool {
        Self::CACHEBUF_LEN == self.len
    }

    #[inline(always)]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len()]
    }

    pub(crate) unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        &mut self.buf[..len]
    }
}

impl CacheBuf {
    #[inline(always)]
    pub(crate) fn push(&mut self, b: u8) {
        self.try_push(b).unwrap()
    }

    #[inline(always)]
    pub(crate) fn try_push(&mut self, b: u8) -> Result<(), CapacityError> {
        if self.is_full() {
            Err(CapacityError)
        } else {
            unsafe {
                self.push_unchecked(b);
            }

            Ok(())
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn push_unchecked(&mut self, b: u8) {
        let index = self.len();
        *(self.buf.get_unchecked_mut(index as usize)) = b;
        self.len -= 1;
    }

    pub(crate) fn extend_from_slice(&mut self, slice: &[u8]) {
        self.try_extend_from_slice(slice).unwrap()
    }

    #[inline]
    pub(crate) fn try_extend_from_slice(&mut self, slice: &[u8]) -> Result<(), CapacityError> {
        if slice.len() > self.remaining_capacity() {
            Err(CapacityError)
        } else {
            unsafe {
                self.extend_from_slice_unchecked(slice);
            }

            Ok(())
        }
    }

    #[inline]
    pub(crate) unsafe fn extend_from_slice_unchecked(&mut self, slice: &[u8]) {
        let index = self.len();

        self.buf
            .get_unchecked_mut(index..index + slice.len())
            .copy_from_slice(slice);

        self.len -= slice.len() as u8;
    }

    #[inline]
    pub(crate) fn truncate(&mut self, new_len: usize) {
        self.len = Self::CACHEBUF_LEN - (new_len as u8);
    }
}
