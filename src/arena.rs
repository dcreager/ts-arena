// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, ts-arena authors.
// Please see the COPYING file in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use std::marker::PhantomData;
use std::sync::Arc;

use parking_lot::RwLock;

const CHUNK_SIZE: usize = 32;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Handle<T> {
    index: u32,
    _phantom: PhantomData<T>,
}

impl<T> Handle<T> {
    fn chunk_index(self) -> usize {
        (self.index as usize) / CHUNK_SIZE
    }

    fn element_index(self) -> usize {
        (self.index as usize) % CHUNK_SIZE
    }
}

struct Chunk<T> {
    elements: RwLock<[T; CHUNK_SIZE]>,
}

pub struct Arena<T> {
    inner: Arc<RwLock<Inner<T>>>,
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena {
            inner: Arc::new(RwLock::new(Inner { chunks: Vec::new() })),
        }
    }
}

struct Inner<T> {
    chunks: Vec<Chunk<T>>,
}

impl<T> Inner<T>
where
    T: Copy + Default,
{
    fn new_chunk(&mut self) -> usize {
        let chunk = Chunk {
            elements: [T::default(); CHUNK_SIZE],
        };
        let new_index = self.chunks.len();
        self.chunks.push(chunk);
        new_index
    }
}
