# 🦀 midas_core_rust_jpmc

A Rust-based backend reimplementation of the **JPMorgan Chase Forage Virtual Experience Project**.  
This project is being rebuilt entirely in Rust (instead of Java) to strengthen backend systems programming and async fundamentals using Actix-web and SQLx.

---

## 📦 Tech Stack

- ⚙️ **Rust**
- ⚡ **Actix-Web** – high-performance web framework
- 🧵 **Tokio** – async runtime for non-blocking IO
- 🗃️ **SQLx** – async DB interaction (using SQLite for now)
- 🔐 `.env` for environment config

---

## ✅ Current Features

- [x] `.env`-based DB connection setup
- [x] Async connection to SQLite via `sqlx`
- [x] Local Actix web server setup
- [x] Project modularized (ready for routing, models, services)

---

## 🚀 Planned Features (Coming Soon)

- [ ] Health check endpoint (e.g. `/health`)
- [ ] Stock model + routes
- [ ] Insert & fetch stock data
- [ ] RESTful APIs for core JPM use case
- [ ] PostgreSQL/MongoDB upgrade support
- [ ] Dockerization for deployment

---

## 🛠️ Setup & Run

```bash
git clone https://github.com/ManthanN75/midas_core_rust_jpmc.git
cd midas_core_rust_jpmc

# Add the database file
touch midas.db

# Add your .env
echo "DATABASE_URL=sqlite://midas.db" > .env

# Run the server
cargo run
