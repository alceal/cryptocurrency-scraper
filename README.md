# Cryptocurrency Scraper

## Introduction

The Cryptocurrency Scraper project is a demonstration of web scraping using Rust. It showcases my skills in Rust development by creating a command-line application that extracts cryptocurrency data from CoinMarketCap's website.

## Dependencies

This project relies on several crates from crates.io to achieve its functionality:

- `thirtyfour`: Provides WebDriver client for Selenium, enabling web scraping and interaction with web elements.
- `tokio`: Used for asynchronous execution and handling of asynchronous tasks.
- `webdriver`: Offers functionality for interacting with a WebDriver server, in this case, for Chrome.

## Installation

Clone the repository:

```bash
git clone <https://github.com/alcel/cryptocurrency-scraper.git>
cd cryptocurrency-scraper
```

## Usage

This command will initiate the scraper, which will navigate to CoinMarketCap's website, extract data from the cryptocurrency table, and display the information in the terminal.

```bash
cargo run
```

## Note

- This project is intended for educational and demonstration purposes only.
- Ensure compliance with website terms of service and legalities while using web scraping tools.
- The scraping behavior and functionality might vary based on website changes or updates.
