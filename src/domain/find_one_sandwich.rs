use super::sandwich::{Sandwich, SandwichType};

#[derive(Debug)]
pub enum FindOneError {
    Unknown(String),
    NotFound,
}

pub fn find_one_sandwich<'a>(
    id: &'a str,
    name: &'a str,
    ingredients: &'a Vec<&str>,
) -> Result<Sandwich, FindOneError> {
    let ingredients = ingredients
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();

    let sandwich = Sandwich::new(
        id.to_string(),
        name.to_string(),
        ingredients,
        SandwichType::Meat,
    )
    .unwrap();

    Ok(sandwich)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::string_vec_to_vec_str;
    use crate::tests::test_utils::shared::{
        assert_on_sandwich, stub_ingredients, stub_sandwich, SANDWICH_NAME,
    };

    #[test]
    fn should_create_the_expected_sandwich() {
        let ingredients = stub_ingredients();
        let ingredients = string_vec_to_vec_str(&ingredients);

        match find_one_sandwich("", SANDWICH_NAME, &ingredients) {
            Ok(s) => assert_on_sandwich(stub_sandwich(false), &s, false),
            Err(_) => unreachable!(),
        }
    }
}
