use super::{new_element, Element, Encode, EncodeError};

impl Encode for crate::svd::Access {
    type Error = EncodeError;

    fn encode(&self) -> Result<Element, EncodeError> {
        Ok(new_element("access", Some(self.to_str().to_string())))
    }
}
