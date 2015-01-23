// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.


//
// FieldMask
//


pub trait FieldMask<T> {
  fn field_mask (&self) -> T;
}


//
// FromNative
//


pub trait FromNative<T> {
  fn from_native (T) -> Self;
}


//
// ToNative
//


pub trait ToNative<T> {
  fn to_native (&self) -> T;
}
