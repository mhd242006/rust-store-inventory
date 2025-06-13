# 📦 Rust Store Inventory Manager

A simple command-line application written in Rust to manage a small store's inventory.  
This project demonstrates file I/O, user input, and JSON serialization.

---

## ✅ Features:
- Add items with quantity
- Display all items
- Show total quantity of all items
- Export data to `items.txt` (human-readable) and `items.json` (structured)
- Load and display data from the JSON file

---

## 🛠 Technologies Used:
- Rust
- [Serde](https://serde.rs/) for serialization
- `text_io` for reading CLI input
- Standard library I/O (`std::fs`, `std::io`)

---

## 📁 Output Files:
- `items.txt`: Human-readable list of added items
- `items.json`: Structured data for future processing or loading

---

## 🎯 Usage Example:
```bash
1. Add an item to store
2. Show all items
3. Show total quantity
4. Show items from JSON file
5. Exit
