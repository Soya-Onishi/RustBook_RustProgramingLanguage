use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T> 
    where 
        T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>
}

impl<T> Cacher<T> 
    where 
        T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {            
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cacher = Cacher::new(|intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cacher.value(intensity)
        );

        println!(
            "Next, do {} situps!",
            cacher.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cacher.value(intensity)
            )
        }
    }
}