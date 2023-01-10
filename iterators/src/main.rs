struct Stepper {
    curr: i32,
    step: i32,
    max: i32
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.max {
            return None
        }
        self.curr = self.curr + self.step;
        Some(self.curr)
    }
}

fn main() {

    let mut _stepper = Stepper {
        curr: 0,
        step: 2,
        max: 200
    };
    

    /*
        For loop implementations of stepper
    */
    for i in &mut _stepper {
        println!("For Loop implementation {}", i)
    }
    
    


    _stepper.curr = 0; // resetting the current value of stepper
    /*
        While loop implementation
    */
    while _stepper.curr < _stepper.max {
        _stepper.next();
        println!("While Loop Implementaition, {}", _stepper.curr)
    }

    _stepper.curr = 0; // resetting the current value of stepper'

    while let Some(n) = _stepper.next() {
        println!("While loop two:: {}", n)
    }

    /*
        "loop" loop implementation 
    */
    _stepper.curr = 0; // resetting the current value of stepper
    loop {
        match &mut _stepper.next() {
            Some(v) => println!("Loop Implementation::{}", v),
            None => break
        }
    }
}
