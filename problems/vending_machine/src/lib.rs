#[derive(Debug)]
struct Stock {
    coke: u32,
    soda: u32,
    water: u32,
    fanta: u32,
}

#[derive(Debug, PartialEq)]
enum Beverage {
    Coke,
    Soda,
    Water,
    Fanta,
}

#[derive(Debug)]
struct VendingMachine {
    inserted_coin: u32,
    stock: Stock,
}

impl VendingMachine {
    fn new() -> Self {
        VendingMachine {
            inserted_coin: 0u32,
            stock: Stock {
                coke: 10,
                soda: 10,
                water: 10,
                fanta: 10,
            },
        }
    }

    fn insert_coin(&mut self, coin: u32) {
        unimplemented!()
    }

    fn buy(&mut self, beverage: &Beverage) -> Option<Beverage> {
        unimplemented!()
    }

    fn refill(&mut self) {
        unimplemented!()
    }

    fn get_beverage_price(&self, bev: &Beverage) -> u32 {
        match bev {
            Beverage::Coke => 1200,
            Beverage::Soda => 1100,
            Beverage::Water => 800,
            Beverage::Fanta => 1300,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_insert_coin() {
        let mut vm = VendingMachine::new();

        vm.insert_coin(500);

        assert_eq!(500, vm.inserted_coin);
    }

    #[test]
    fn test_buy_a_coke() {
        let mut vm = VendingMachine::new();

        let price = vm.get_beverage_price(&Beverage::Coke);

        vm.insert_coin(price);

        let beverage = vm.buy(&Beverage::Coke);

        assert_eq!(0, vm.inserted_coin);
        assert_eq!(9, vm.stock.coke);
        assert_eq!(beverage, Some(Beverage::Coke));
    }

    #[test]
    fn test_buy_a_soda() {
        let mut vm = VendingMachine::new();

        let price = vm.get_beverage_price(&Beverage::Soda);

        vm.insert_coin(price);

        let beverage = vm.buy(&Beverage::Soda);

        assert_eq!(0, vm.inserted_coin);
        assert_eq!(9, vm.stock.soda);
        assert_eq!(beverage, Some(Beverage::Soda));
    }

    #[test]
    fn test_buy_a_water() {
        let mut vm = VendingMachine::new();

        let price = vm.get_beverage_price(&Beverage::Water);

        vm.insert_coin(price);

        let beverage = vm.buy(&Beverage::Water);

        assert_eq!(0, vm.inserted_coin);
        assert_eq!(9, vm.stock.water);
        assert_eq!(beverage, Some(Beverage::Water));
    }

    #[test]
    fn test_buy_a_fanta() {
        let mut vm = VendingMachine::new();

        let price = vm.get_beverage_price(&Beverage::Fanta);

        vm.insert_coin(price);

        let beverage = vm.buy(&Beverage::Fanta);

        assert_eq!(0, vm.inserted_coin);
        assert_eq!(9, vm.stock.fanta);
        assert_eq!(beverage, Some(Beverage::Fanta));
    }

    #[test]
    fn test_cannot_buy_out_of_stock_one() {
        let mut vm = VendingMachine::new();

        let water_price = vm.get_beverage_price(&Beverage::Water);

        vm.insert_coin(water_price * 11);

        // 10번 구매
        for _ in 0..11 {
            vm.buy(&Beverage::Water);
        }

        let bev = vm.buy(&Beverage::Water);

        assert_eq!(bev, None);
    }

    #[test]
    fn test_refill_stocks() {
        let mut vm = VendingMachine::new();

        let coke_price = vm.get_beverage_price(&Beverage::Coke);
        let soda_price = vm.get_beverage_price(&Beverage::Soda);
        let water_price = vm.get_beverage_price(&Beverage::Water);
        let fanta_price = vm.get_beverage_price(&Beverage::Fanta);

        vm.insert_coin(coke_price * 4);
        vm.insert_coin(soda_price * 3);
        vm.insert_coin(water_price * 3);
        vm.insert_coin(fanta_price * 5);

        // 콜라 4번 구매
        for _ in 0..4 {
            vm.buy(&Beverage::Coke);
        }

        // 소다 3번 구매
        for _ in 0..3 {
            vm.buy(&Beverage::Soda);
        }

        // 물 3번 구매
        for _ in 0..3 {
            vm.buy(&Beverage::Water);
        }

        // 환타 5번 구매
        for _ in 0..5 {
            vm.buy(&Beverage::Fanta);
        }

        vm.refill();

        assert_eq!(vm.stock.coke, 10);
        assert_eq!(vm.stock.soda, 10);
        assert_eq!(vm.stock.water, 10);
        assert_eq!(vm.stock.fanta, 10);
    }
}
