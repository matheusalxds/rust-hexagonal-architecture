#[derive(Debug)]
pub enum DeleteOneError {
    Unknown(String),
}

pub fn delete_one_sandwich(id: &str) -> Result<(), DeleteOneError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_utils::shared::SANDWICH_ID;

    #[test]
    fn should_delete_a_sandwich() {
        match delete_one_sandwich(SANDWICH_ID) {
            Ok(()) => {}
            _ => unreachable!(),
        }
    }
}
