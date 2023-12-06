use thirtyfour::prelude::*;

const URL: &str = "https://coinmarketcap.com";

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    let _ = caps.set_headless();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto(URL).await?;

    let table = driver.find(By::ClassName("cmc-table")).await?;

    println!("# - Name - Symbol - Price");

    let rows = table.find_all(By::Css("tbody>tr")).await?;

    for row in &rows[..10] {
        let cells = row.find_all(By::Css("p, span")).await?;

        let mut values: Vec<String> = Vec::new();

        for cell in cells.iter() {
            let value = cell.text().await?;
            values.push(value);
        }

        let chosen_values = values
            .iter()
            .filter(|&i| !i.is_empty())
            .map(|i| i.to_owned())
            .collect::<Vec<String>>()
            .iter()
            .take(4)
            .map(|i| i.to_owned())
            .collect::<Vec<String>>()
            .join(" - ");

        println!("{chosen_values}");
    }

    driver.quit().await?;

    Ok(())
}
