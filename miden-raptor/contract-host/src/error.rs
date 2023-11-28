use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum MidenError{
    InvalidProof = 0
}
impl From<MidenError> for ApiError{
    fn from(e: MidenError) -> Self{
        ApiError::User(e as u16)
    }
}