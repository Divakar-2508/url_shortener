# CompactLinks

A lightweight, performant, and easy-to-use URL shortening service built using Rust and Axum. CompactLinks allows users to convert long URLs into compact, shareable links while providing features like URL management and redirection.

---

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Project Roadmap](#project-roadmap)
- [Contributing](#contributing)
- [License](#license)

---

## Introduction

CompactLinks is inspired by modern URL shortening services and aims to provide a simple yet powerful solution for developers and users alike. With a minimalistic design and robust backend, it is suitable for personal projects, learning, and production use.

---

## Features

- Generate short and unique links for long URLs.
- Redirect users to the original URL using the shortened link.
- URL management through API.
- Built using Rust’s asynchronous web framework Axum.
- Lightweight and fast with SQLite as the database backend.

---

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [SQLite](https://www.sqlite.org/download.html) (optional if using an in-memory database for development)

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/compactlinks.git
   cd compactlinks
   ```
2. Install dependencies:
   ```bash
   cargo build
   ```
3. Run the application:
   ```bash
   cargo run
   ```

The server will start at `http://localhost:3000` by default.

---

## Usage

### API Endpoints

1. **Shorten a URL**
   - Endpoint: `POST /shorten`
   - Request Body:
     ```json
     {
       "url": "https://example.com"
     }
     ```
   - Response:
     ```json
     {
       "id": 1,
       "url": "https://example.com",
       "short_code": "abc123",
       "created_at": "2024-12-11T12:00:00Z",
       "updated_at": "2024-12-11T12:00:00Z"
     }
     ```

2. **Retrieve URL Details**
   - Endpoint: `GET /shorten/:short_code`
   - Response:
     ```json
     {
       "id": 2,
       "url": "https://example.com",
       "short_code": "abc123",
       "created_at": "2024-12-11T12:00:00Z",
       "updated_at": "2024-12-11T12:00:00Z"
     }
     ```

3. **Redirect to Original URL**
   - Endpoint: `GET /r/:short_code`
   - Response: HTTP 302 redirect to the original URL.

---

## Project Roadmap

The roadmap for CompactLinks follows [this comprehensive guide](https://roadmap.sh/projects/url-shortening-service):

## Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue for discussion. Please ensure your changes include appropriate tests and documentation updates.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Enjoy using CompactLinks! Shorten, share, and manage your URLs effortlessly.

