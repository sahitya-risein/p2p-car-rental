

# 🚗 P2P Car Rental DApp on Soroban vv

A decentralized **peer-to-peer car rental platform** built using **Soroban on Stellar**, enabling users to rent cars securely without intermediaries.

---

## 📌 Overview

This project allows car owners to list their vehicles and renters to book them in a **trustless and transparent environment** powered by blockchain.

### ✨ Key Highlights

* Decentralized car listings
* Secure rental lifecycle
* Transparent pricing
* No middlemen
* Built for scalability and real-world adoption

---

## 🏗️ Smart Contract Features

### 🚘 Car Management

* Add car listings
* Set price per day
* Toggle availability

### 📅 Rental System

* Rent available cars
* Track rental duration
* Mark rentals as completed

### 🔐 Access Control

* Owner authorization for listing
* Renter authorization for booking

---

## 📂 Project Structure

```
├── contracts/
│   └── car_rental.rs     # Soroban smart contract
├── README.md
```

---

## ⚙️ Tech Stack

* **Smart Contracts:** Rust
* **Blockchain:** Stellar
* **Framework:** Soroban SDK

---

## 🚀 How It Works

### 1️⃣ List a Car

Car owners register their vehicle with:

* Price per day
* Ownership details
* Availability status

### 2️⃣ Rent a Car

Renters:

* Select a car
* Choose number of days
* Initiate rental

### 3️⃣ Return Car

* Rental is marked complete
* Car becomes available again

---

## 🧠 Core Functions

| Function     | Description          |
| ------------ | -------------------- |
| `add_car`    | Register a new car   |
| `rent_car`   | Book a car           |
| `return_car` | End rental           |
| `get_car`    | Fetch car details    |
| `get_rental` | Fetch rental details |

---

## ⚠️ Current Limitations

* ❌ No payment integration yet
* ❌ No escrow mechanism
* ❌ No time-based enforcement
* ❌ No reputation system

---

## 🔮 Future Improvements

### 💰 Payments & Escrow

* Integrate Stellar token transfers (XLM / USDC)
* Add escrow-based payment system

### ⏱️ Time Enforcement

* Use ledger timestamps for rental duration

### ⭐ Reputation System

* Ratings & reviews for users

### 🛡️ Security Enhancements

* Security deposits
* Late return penalties

### 🌍 Real-World Integration

* IPFS for car metadata (images, details)
* Location-based discovery

---

## 🧪 Example Use Case

> Alice lists her car for 10 XLM/day → Bob rents it for 3 days → Contract locks booking → Bob returns car → Car becomes available again.

---

## 💡 Vision

To build the **“Airbnb for cars” on blockchain**, leveraging **Stellar** for fast, low-cost, and global transactions.

---

## 🤝 Contributing

Contributions are welcome!
Feel free to fork the repo and submit a PR 🚀

---

## 📜 License

MIT License

---

## 🙌 Acknowledgements

* Stellar Development Foundation
* Soroban ecosystem

---

If you want, I can also:

* Turn this into a **GitHub-ready polished repo (with badges, screenshots, demo GIFs)**
* Add a **killer hackathon pitch section (judges LOVE this)**
* Or include **architecture diagrams + flow charts**
