
use std::ops::Drop;
use std::time::Instant;

pub struct ScopeClock {
    scope: &'static str,
    in_time: Instant
}

impl ScopeClock {
    pub fn new(scope: &'static str) -> ScopeClock {
        return ScopeClock {
            scope: scope,
            in_time: Instant::now()
        };
    }

}

impl Drop for ScopeClock {
    fn drop(&mut self) {
        let out_time = Instant::now();
        let duration = out_time - self.in_time;
        println!("{}={}ns", self.scope, duration.as_nanos());
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        use crate::ScopeClock;

        let _sc = ScopeClock::new("my_scope");
        let x = 3;
        let y = 20;
        let result = x * y;
        println!("Result is {}", result);

        assert!(true);
    }

    #[test]
    fn multi_scope() {
        use crate::ScopeClock;

        {
            let _sc = ScopeClock::new("scope_1");
            let x = 3;
            let y = 20;
            let result = x * y;
            println!("Result 1 is {}", result);
        }

        {
            let _sc = ScopeClock::new("scope_2");
            let x = 33;
            let y = 7;
            let result = x % y;
            println!("Result 2 is {}", result);
        }

        assert!(true);
    }

    #[test]
    fn nested_scope() {
        use crate::ScopeClock;

        {
            let _sc = ScopeClock::new("outer_scope");
            let x = 3;
            let y = 20;
            let result = x * y;
            {
                let _sc = ScopeClock::new("inner_scope");
                let x = 33;
                let y = 7;
                let result = x % y;
                println!("Inner result is {}", result);
            }
            println!("Outer result is {}", result);
        }

        assert!(true);
    }
}
