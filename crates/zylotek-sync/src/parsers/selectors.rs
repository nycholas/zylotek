use anyhow::{Result, anyhow, bail};
use scraper::{ElementRef, Selector};

pub(crate) fn query(value: &str) -> Result<Selector> {
    Selector::parse(value).map_err(|_| anyhow!("invalid CSS selector: {value}"))
}

pub(crate) fn text(element: ElementRef<'_>) -> String {
    element
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn table_rows_by_header(
    container: ElementRef<'_>,
    header: &str,
) -> Result<Vec<Vec<String>>> {
    let th_selector = query("th")?;
    let tr_selector = query("tr")?;
    let td_selector = query("td")?;
    let table_selector = query("table")?;

    for table in container.select(&table_selector) {
        let mut column_start = None;
        let mut column_count = 1;
        let mut current_column = 0;

        for th in table.select(&th_selector) {
            let th_text = th
                .text()
                .collect::<String>()
                .trim()
                .to_lowercase()
                .to_string();

            let colspan: usize = th
                .value()
                .attr("colspan")
                .unwrap_or("1")
                .parse()
                .unwrap_or(1);

            if th_text == header.to_lowercase() {
                column_start = Some(current_column);
                column_count = colspan;
                break;
            }

            current_column += colspan;
        }

        let column_start = match column_start {
            Some(value) => value,
            None => continue,
        };

        let mut result = Vec::new();

        for tr in table.select(&tr_selector) {
            let cells: Vec<String> = tr
                .select(&td_selector)
                .map(|td| td.text().collect::<String>().trim().to_string())
                .collect();

            let values = cells
                .iter()
                .skip(column_start)
                .take(column_count)
                .cloned()
                .collect::<Vec<String>>();

            if values.is_empty() {
                continue;
            }

            result.push(values);
        }

        return Ok(result);
    }

    bail!("table column with header {header} not found")
}

pub(crate) fn table_rows_by_heading(
    container: ElementRef<'_>,
    heading: &str,
) -> Result<Vec<Vec<String>>> {
    let row_selector = query("tr")?;
    let cell_selector = query("th, td")?;
    let h3_selector = query("h3")?;
    let table_selector = query("table")?;

    for h3 in container.select(&h3_selector) {
        let title = h3
            .text()
            .collect::<String>()
            .trim()
            .to_lowercase()
            .to_string();

        if title == heading.to_lowercase() {
            let parent = h3.parent().unwrap();
            let parent = scraper::ElementRef::wrap(parent).unwrap();

            return Ok(table_rows(
                parent.select(&table_selector).next().unwrap(),
                &row_selector,
                &cell_selector,
            ));
        }
    }

    bail!("table {} not found", heading)
}

pub(crate) fn table_rows(
    table: ElementRef<'_>,
    row_selector: &Selector,
    cell_selector: &Selector,
) -> Vec<Vec<String>> {
    table
        .select(row_selector)
        .map(|row| table_cell_texts(row, cell_selector))
        .filter(|row| !row.is_empty())
        .collect()
}

fn table_cell_texts(element: ElementRef<'_>, selector: &Selector) -> Vec<String> {
    element
        .select(selector)
        .map(text)
        // .filter(|value| !value.is_empty())
        .collect()
}
