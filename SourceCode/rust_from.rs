/*
Topic : Rust From
*/
#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        // self는 말 그대로 자기 자신을 의미
        // Self는 자기 자신과 타입이 같은 다른 대상을 의미
        Self {
            name: name.to_string(),
            population
        }
    }
}

// Country::from(vec![City, City]);
impl From<Vec<City>> for Country { 
    // 라이브러리에 존재하는 trait인 From<T>를 가져와 Vec<City>를 받을 수 있도록 하고
    // 이를 Country 구조체에서 사용할 수 있게 함
    fn from(cities: Vec<City>) -> Self { // 소유권 이전 발생
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}", city.name, city.population);
        }
    }
}

fn main() {
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    let finland = Country::from(finland_cities);
    // let finland: Country = finland_cities.into(); 
    // 위 코드는 .from()을 사용한 경우와 같음 하지만 let에 미리 타입을 지정해주어야함
    // .from()을 impl하면 결국 .into()도 lmpl하게 되기 때문에 가능함
    finland.print_cities();
}