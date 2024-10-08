use crate::{Field, FieldSerde};

/// Configurations for the SimdField.
pub trait SimdField: From<Self::Scalar> + Field + FieldSerde {
    /// Field for the challenge. Can be self.
    type Scalar: Field + FieldSerde + Send;

    /// scale self with the challenge
    fn scale(&self, challenge: &Self::Scalar) -> Self;

    /// pack a vec of scalar field into self
    fn pack(base_vec: &[Self::Scalar]) -> Self;

    /// unpack into a vector.
    fn unpack(&self) -> Vec<Self::Scalar>;

    fn pack_size() -> usize;
}
