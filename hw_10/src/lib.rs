use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cat {
    age: isize,
    name: String,
}

impl Cat {
    pub fn new(age: isize, name: &str) -> Self {
        Self {
            age,
            name: name.to_string(),
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cat is {} years old and it's name is {}",
            self.age, self.name
        )
    }
}

impl Add<isize> for Cat {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        let new_age = self.age + rhs;

        Self {
            age: new_age,
            name: self.name,
        }
    }
}

impl AddAssign<isize> for Cat {
    fn add_assign(&mut self, rhs: isize) {
        self.age = self.age + rhs
    }
}

#[derive(Debug, PartialEq)]
enum Pet {
    Cat { age: isize, name: String },
    _Dog,
}

impl From<Cat> for Pet {
    fn from(cat: Cat) -> Self {
        Pet::Cat {
            age: cat.age,
            name: cat.name,
        }
    }
}

impl TryInto<Cat> for Pet {
    type Error = String;

    fn try_into(self) -> Result<Cat, Self::Error> {
        if let Pet::Cat { age, name } = self {
            Ok(Cat::new(age, name.as_str()))
        } else {
            Err("pet is not a cat".to_string())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_name_works() {
        let name = "Kitty";
        let cat = Cat::new(10, name);

        let expected = name;
        let actual = cat.get_name();

        assert_eq!(expected, actual)
    }

    #[test]
    fn add_works() {
        let cat = Cat::default();

        let expected = 1;
        let actual = cat + 1;

        assert_eq!(expected, actual.age)
    }

    #[test]
    fn add_assign_works() {
        let mut cat = Cat::default();

        let expected = 1;
        cat += 1;

        assert_eq!(expected, cat.age)
    }

    #[test]
    fn into_for_pet_works() {
        let pet = Pet::Cat {
            age: 0,
            name: "".to_string(),
        };

        let expected = Cat::default();
        let actual = pet.try_into();

        assert_eq!(expected, actual.unwrap())
    }

    #[test]
    fn into_for_cat_works() {
        let cat = Cat::default();

        let expected = Pet::Cat {
            age: 0,
            name: "".to_string(),
        };
        let actual = cat.into();

        assert_eq!(expected, actual)
    }
}
