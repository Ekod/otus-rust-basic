trait SomeTrait: Default + PartialEq<Self> {
    fn get_item(&self, item: Item) -> f64;
    fn set_item(&mut self, item: Item, value: f64);

    fn is_default(&self) -> bool {
        Self::default() == *self
    }
    fn sum(&self) -> f64 {
        self.get_item(Item::First) + self.get_item(Item::Second) + self.get_item(Item::Third)
    }
}

pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

#[derive(Default, PartialEq)]
pub struct Tuple(u32, f32, f64);

impl SomeTrait for Tuple {
    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }
}

#[derive(Default, PartialEq)]
pub struct Array([f64; 3]);

impl SomeTrait for Array {
    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }
    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generic_sum<T: SomeTrait>(item: &T) -> f64 {
        item.sum()
    }

    fn generic_is_default<T: SomeTrait>(item: &T) -> bool {
        item.is_default()
    }

    fn get_item(t: &impl SomeTrait, item: Item) -> f64 {
        t.get_item(item)
    }

    fn set_item(t: &mut impl SomeTrait, item: Item, value: f64) {
        t.set_item(item, value);
    }

    #[test]
    fn sum_success() {
        let arr = Array([1., 1., 1.]);
        let expected = 3.;
        let actual = generic_sum(&arr);

        assert_eq!(expected, actual);

        let tup = Tuple(1, 1., 1.);
        let expected = 3.;
        let actual = generic_sum(&tup);

        assert_eq!(expected, actual);
    }

    #[test]
    fn is_default_success() {
        let arr = Array::default();
        let expected = true;
        let actual = generic_is_default(&arr);

        assert_eq!(expected, actual);

        let tup = Tuple::default();
        let expected = true;
        let actual = generic_is_default(&tup);

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_item_success() {
        let arr = Array([2., 3., 4.]);
        let expected = 2.;
        let actual = get_item(&arr, Item::First);

        assert_eq!(expected, actual);

        let tup = Tuple(2, 3., 4.);
        let expected = 3.;
        let actual = get_item(&tup, Item::Second);

        assert_eq!(expected, actual);
    }

    #[test]
    fn set_item_success() {
        let mut arr = Array([2., 3., 4.]);
        set_item(&mut arr, Item::First, 22.);

        let expected = 22.;
        let actual = arr.0[0];

        assert_eq!(expected, actual);

        let mut tup = Tuple(2, 3., 4.);
        set_item(&mut tup, Item::Second, 55.);

        let expected = 55.;
        let actual = tup.1;

        assert_eq!(expected, actual);
    }
}
