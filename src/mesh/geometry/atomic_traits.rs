//! This module contains traits that define the shared behaviour between points and vectors of various dimensions.
//!

////////////////////////////////////////////////////////////////////////////////
// Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub trait Atomic<'a> {
    fn x(&'a self) -> f64 {
        0.0
    }
    fn y(&'a self) -> f64 {
        0.0
    }
    fn z(&'a self) -> f64 {
        0.0
    }
    fn w(&'a self) -> f64 {
        0.0
    }
}

pub trait Atomic1D<'a>: Atomic<'a> {
    fn mut_x(&'a mut self) -> &'a mut f64;
}

pub trait Atomic2D<'a>: Atomic1D<'a> {
    fn mut_y(&'a mut self) -> &'a mut f64;
}

pub trait Atomic3D<'a>: Atomic2D<'a> {
    fn mut_z(&'a mut self) -> &'a mut f64;
}

pub trait Atomic4D<'a>: Atomic3D<'a> {
    fn mut_w(&'a mut self) -> &'a mut f64;
}

////////////////////////////////////////////////////////////////////////////////
// Macros //////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
#[macro_export]
macro_rules! impl_atomic {
    (impl Atomic1D for $type_: ident) => {
        impl Atomic<'_> for $type_ {
            fn x(&self) -> f64 {
                self.0[0]
            }
        }
        impl_atomic_helper! {impl Atomic1D for $type_}
    };

    (impl Atomic2D for $type_: ident) => {
        impl Atomic<'_> for $type_ {
            fn x(&self) -> f64 {
                self.0[0]
            }
            fn y(&self) -> f64 {
                self.0[1]
            }
        }
        impl_atomic_helper! {impl Atomic1D for $type_}
        impl_atomic_helper! {impl Atomic2D for $type_}
    };

    (impl Atomic3D for $type_: ident) => {
        impl Atomic<'_> for $type_ {
            fn x(&self) -> f64 {
                self.0[0]
            }
            fn y(&self) -> f64 {
                self.0[1]
            }
            fn z(&self) -> f64 {
                self.0[2]
            }
        }
        impl_atomic_helper! {impl Atomic1D for $type_}
        impl_atomic_helper! {impl Atomic2D for $type_}
        impl_atomic_helper! {impl Atomic3D for $type_}
    };

    (impl Atomic4D for $type_: ident) => {
        impl Atomic<'_> for $type_ {
            fn x(&self) -> f64 {
                self.0[0]
            }
            fn y(&self) -> f64 {
                self.0[1]
            }
            fn z(&self) -> f64 {
                self.0[2]
            }
            fn w(&self) -> f64 {
                self.0[3]
            }
        }
        impl_atomic_helper! {impl Atomic1D for $type_}
        impl_atomic_helper! {impl Atomic2D for $type_}
        impl_atomic_helper! {impl Atomic3D for $type_}
        impl_atomic_helper! {impl Atomic4D for $type_}
    };
}

#[macro_export]
macro_rules! impl_atomic_helper {
    (impl Atomic1D for $type_: ident) => {
        impl Atomic1D<'_> for $type_ {
            fn mut_x(&mut self) -> &mut f64 {
                &mut self.0[0]
            }
        }
    };
    (impl Atomic2D for $type_: ident) => {
        impl Atomic2D<'_> for $type_ {
            fn mut_y(&mut self) -> &mut f64 {
                &mut self.0[1]
            }
        }
    };

    (impl Atomic3D for $type_: ident) => {
        impl Atomic3D<'_> for $type_ {
            fn mut_z(&mut self) -> &mut f64 {
                &mut self.0[2]
            }
        }
    };

    (impl Atomic4D for $type_: ident) => {
        impl Atomic4D<'_> for $type_ {
            fn mut_w(&mut self) -> &mut f64 {
                &mut self.0[3]
            }
        }
    };
}
