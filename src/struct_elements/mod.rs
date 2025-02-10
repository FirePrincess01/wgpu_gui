//! Helper functions to visit certain elements of a struct


pub trait BInterface {
    fn calculate_sum(&mut self, elements: &mut dyn FnMut(&mut StructElements));
}

pub struct StructElementB<'a> {
    visitor: &'a mut dyn FnMut(&mut dyn BInterface, &mut dyn FnMut(&mut StructElements)),
}

impl<'a> StructElementB<'a> {

    pub fn new(visitor: &'a mut dyn FnMut(&mut dyn BInterface, &mut dyn FnMut(&mut StructElements))) -> Self {
        Self { visitor }
    }
    
    pub fn use_b(&mut self, b_interface: &mut dyn BInterface, elements: &mut dyn FnMut(&mut StructElements)) {
        (self.visitor)(b_interface, elements);
    }
}

pub struct StructElements<'a> {
    visitor: &'a mut dyn FnMut(&mut u32),
}

impl<'a> StructElements<'a> {
    
    pub fn new(visitor: &'a mut dyn FnMut(&mut u32)) -> Self {
        Self { visitor }
    }
    
    pub fn add(&mut self, mut elem: u32) {
        (self.visitor)(&mut elem);
    }
}




#[cfg(test)]
mod tests {
    use super::{BInterface, StructElementB, StructElements};

    struct A {
        a: u32,
        b: u32,
        c: u32,
        struct_b: B,
    }

    impl A {
        fn get_elements(&mut self, struct_elements: &mut StructElementB) {
            struct_elements.use_b(&mut self.struct_b, &mut |struct_elements: &mut StructElements| {
                struct_elements.add(self.a);
                struct_elements.add(self.b);
                struct_elements.add(self.c);
            });
        }
    }

    struct B {
        sum: u32,
    }

    impl BInterface for B {
        fn calculate_sum(&mut self, elements: &mut dyn FnMut(&mut StructElements)) {
            let mut sum_visitor = |elem: &mut u32| {
                self.sum += *elem;
            };

            let mut struct_elements = StructElements::new(&mut sum_visitor);

            elements(&mut struct_elements);
        }
    }


    #[test]
    fn test_simple() {
        let mut a = A {
            a: 1,
            b: 2,
            c: 3,
            struct_b: B{ sum: 0 },
        };

        let mut func = | b_interface: &mut dyn BInterface, mut elements: &mut dyn FnMut(&mut StructElements) | {
            b_interface.calculate_sum(&mut elements);
        };

        let mut struct_element_b = StructElementB::new(&mut func);
        a.get_elements(&mut struct_element_b);

        assert_eq!(a.struct_b.sum, 6);

    }
}