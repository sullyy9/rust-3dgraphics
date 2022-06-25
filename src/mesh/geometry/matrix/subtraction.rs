//! Implementation of matrix subtraction.
//!

use std::ops::{Sub, SubAssign};

use super::Matrix;

macro_rules! add_impl {
    ({$lhs_t:ty} - {$rhs_t:ty}) => {
        impl<const R: usize, const C: usize> Sub<$rhs_t> for $lhs_t {
            type Output = Matrix<R, C>;

            fn sub(self, rhs: $rhs_t) -> Self::Output {
                let mut mat = self.clone();
                mat.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
                mat
            }
        }
    };
    ({$lhs_t:ty} -= {$rhs_t:ty}) => {
        impl<const R: usize, const C: usize> SubAssign<$rhs_t> for $lhs_t {
            fn sub_assign(&mut self, rhs: $rhs_t) {
                self.iter_mut()
                .zip(rhs.iter())
                .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
            }
        }
    };
}

add_impl! {{Matrix<R, C>} - {Matrix<R, C>}}
add_impl! {{Matrix<R, C>} - {&Matrix<R, C>}}
add_impl! {{&Matrix<R, C>} - {Matrix<R, C>}}
add_impl! {{&Matrix<R, C>} - {&Matrix<R, C>}}

add_impl! {{Matrix<R, C>} -= {Matrix<R, C>}}
add_impl! {{Matrix<R, C>} -= {&Matrix<R, C>}}