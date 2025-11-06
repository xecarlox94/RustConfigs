#![feature(negative_impls)]

use std::{marker::PhantomData, mem::ManuallyDrop};


struct MyType;

impl !Drop for MyType {
}

pub struct DropWhenItsCold
{
    i: (),
_m: MyType
}

impl DropWhenItsCold {
    pub fn new() -> Self {
        Self { i: (), _m: MyType  }
    }
}

impl DropWhenItsCold {
    async fn shutdown(self) -> DropLikeItsHot {
        DropLikeItsHot {
            i: self.i
        }
    }
}

struct DropLikeItsHot
{
    i: ()
}



