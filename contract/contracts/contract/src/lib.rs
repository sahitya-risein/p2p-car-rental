#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec,
};

// =======================
// Data Structures
// =======================

#[derive(Clone)]
#[contracttype]
pub struct Car {
    pub owner: Address,
    pub price_per_day: i128,
    pub is_available: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct Rental {
    pub renter: Address,
    pub car_id: u32,
    pub days: u32,
    pub is_active: bool,
}

// =======================
// Storage Keys
// =======================

#[contracttype]
pub enum DataKey {
    Car(u32),
    Rental(u32),
    CarCount,
    RentalCount,
}

// =======================
// Contract
// =======================

#[contract]
pub struct CarRentalContract;

#[contractimpl]
impl CarRentalContract {

    // Add a car
    pub fn add_car(env: Env, owner: Address, price_per_day: i128) -> u32 {
        owner.require_auth();

        let mut car_count: u32 = env.storage().instance().get(&DataKey::CarCount).unwrap_or(0);
        car_count += 1;

        let car = Car {
            owner: owner.clone(),
            price_per_day,
            is_available: true,
        };

        env.storage().instance().set(&DataKey::Car(car_count), &car);
        env.storage().instance().set(&DataKey::CarCount, &car_count);

        car_count
    }

    // Rent a car
    pub fn rent_car(env: Env, renter: Address, car_id: u32, days: u32) -> u32 {
        renter.require_auth();

        let mut car: Car = env
            .storage()
            .instance()
            .get(&DataKey::Car(car_id))
            .expect("Car not found");

        if !car.is_available {
            panic!("Car not available");
        }

        car.is_available = false;

        let total_price = car.price_per_day * days as i128;

        // NOTE: Payment logic should be added here using token transfer

        let mut rental_count: u32 = env.storage().instance().get(&DataKey::RentalCount).unwrap_or(0);
        rental_count += 1;

        let rental = Rental {
            renter: renter.clone(),
            car_id,
            days,
            is_active: true,
        };

        env.storage().instance().set(&DataKey::Rental(rental_count), &rental);
        env.storage().instance().set(&DataKey::RentalCount, &rental_count);
        env.storage().instance().set(&DataKey::Car(car_id), &car);

        rental_count
    }

    // Return car
    pub fn return_car(env: Env, renter: Address, rental_id: u32) {
        renter.require_auth();

        let mut rental: Rental = env
            .storage()
            .instance()
            .get(&DataKey::Rental(rental_id))
            .expect("Rental not found");

        if !rental.is_active {
            panic!("Rental already completed");
        }

        if rental.renter != renter {
            panic!("Unauthorized");
        }

        let mut car: Car = env
            .storage()
            .instance()
            .get(&DataKey::Car(rental.car_id))
            .unwrap();

        car.is_available = true;
        rental.is_active = false;

        env.storage().instance().set(&DataKey::Car(rental.car_id), &car);
        env.storage().instance().set(&DataKey::Rental(rental_id), &rental);
    }

    // Get car details
    pub fn get_car(env: Env, car_id: u32) -> Car {
        env.storage()
            .instance()
            .get(&DataKey::Car(car_id))
            .expect("Car not found")
    }

    // Get rental details
    pub fn get_rental(env: Env, rental_id: u32) -> Rental {
        env.storage()
            .instance()
            .get(&DataKey::Rental(rental_id))
            .expect("Rental not found")
    }
}