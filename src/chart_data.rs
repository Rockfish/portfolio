#![allow(dead_code)]

use lopdf::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ChartData {
    symbol: String,
    chart: String,
    update_date: String,
    quality_rank: String,
    shares_outstanding: String,
    institution_own: String,
    div_paid_since: String,
    profit_margin: String,
    ttm_earnings: String,
    pe_ratio: String,
    book_value: String,
    div_payout: String,
    current_price: String,
    current_yield: String,
    overvalue_price: String,
    overvalue_pts_up: String,
    overvalue_yield: String,
    overvalue_percent_up: String,
    undervalue_price: String,
    undervalue_pts_dn: String,
    undervalue_yield: String,
    undervalue_percent_dn: String,
}

enum Flag {
    None,
    Current,
    Overvalue,
    Undervalue,
    DIV,
    EPS
}

impl ChartData {

    pub fn read_data(file: &str) -> Self {
        let mut data = Self::extract_data(file);
        let (symbol, chart_file) = Self::get_symbol_and_chart(file);
        data.symbol = symbol;
        data.chart = chart_file;
        data.update_date = chrono::offset::Local::now().to_string();
        data
    }

    fn get_symbol_and_chart(file: &str) -> (String, String) {
        let start = file.find("__").unwrap();
        let end = file.find("_chart").unwrap();
        let symbol = file[start + 2..end].to_string();
        let start = file.rfind('/').unwrap();
        let chart_file = file[start + 1..].to_string();
        (symbol, chart_file)
    }

    fn extract_data(file: &str) -> Self {
        let text = read_pdf(file).unwrap();
        let lines: Vec<String> = text.split('\n').map(|t| t.to_string() ).collect();

        let mut data = ChartData::default();
        let mut flag = Flag::None;

        for line in lines {
            if line.is_empty() || line.starts_with("Investment") || line.starts_with("http") {
                continue;
            }
            if line.starts_with("Quality") {
                data.quality_rank = line[14..].trim().to_string();
            }
            if line.starts_with("Shares Outstg") {
                data.shares_outstanding = line[19..].trim().to_string();
            }
            if line.starts_with("Inst Own") {
                data.institution_own = line[9..].trim().replace('%', "").to_string();
            }
            if line.starts_with("Div Paid Since:") {
                data.div_paid_since = line[15..].trim().to_string();
            }
            if line.starts_with("Profit Margin:") {
                data.profit_margin = line[14..].trim().replace('%', "").to_string();
            }
            if line.starts_with("TTM Earnings:") {
                data.ttm_earnings = line[13..].trim().replace('$', "").to_string();
            }
            if line.starts_with("P/E Ratio:") {
                data.pe_ratio = line[10..].trim().to_string();
            }
            if line.starts_with("Book Value:") {
                data.book_value = line[11..].trim().to_string();
            }
            if line.starts_with("Div Payout:") {
                data.div_payout = line[11..].trim().replace('%', "").to_string();
            }
            if line.starts_with("CURRENT") {
                flag = Flag::Current;
            }
            if line.starts_with("OVERVALUE") {
                flag = Flag::Overvalue;
            }
            if line.starts_with("UNDERVALUE") {
                flag = Flag::Undervalue;
            }
            if line.starts_with("Price:") {
                let price = line[6..].trim().to_string();
                match flag {
                    Flag::Current => {data.current_price = price}
                    Flag::Overvalue => {data.overvalue_price = price}
                    Flag::Undervalue => {data.undervalue_price = price},
                    _ => {}
                }
            }
            if line.starts_with("Yield:") {
                let value = line[6..].trim().replace('%', "").to_string();
                match flag {
                    Flag::Current => {data.current_yield = value }
                    Flag::Overvalue => {data.overvalue_yield = value }
                    Flag::Undervalue => {data.undervalue_yield = value }
                    _ => {}
                }
            }
            if line.starts_with("Pts Up:") {
                data.overvalue_pts_up = line[7..].trim().to_string();
            }
            if line.starts_with("% Up:") {
                data.overvalue_percent_up = line[5..].trim().replace('%', "").to_string();
            }
            if line.starts_with("Pts Dn:") {
                data.undervalue_pts_dn = line[7..].trim().to_string();
            }
            if line.starts_with("Dn:") {
                data.undervalue_percent_dn = line[3..].trim().replace('%', "").to_string();
            }
        }
        data
    }
}

impl Default for ChartData {
    fn default() -> Self {
        ChartData {
            symbol: "".to_string(),
            chart: "".to_string(),
            update_date: "".to_string(),
            quality_rank: "".to_string(),
            shares_outstanding: "".to_string(),
            institution_own: "".to_string(),
            div_paid_since: "".to_string(),
            profit_margin: "".to_string(),
            ttm_earnings: "".to_string(),
            pe_ratio: "".to_string(),
            book_value: "".to_string(),
            div_payout: "".to_string(),
            current_price: "".to_string(),
            current_yield: "".to_string(),
            overvalue_price: "".to_string(),
            overvalue_pts_up: "".to_string(),
            overvalue_yield: "".to_string(),
            overvalue_percent_up: "".to_string(),
            undervalue_price: "".to_string(),
            undervalue_pts_dn: "".to_string(),
            undervalue_yield: "".to_string(),
            undervalue_percent_dn: "".to_string(),
        }
    }
}

fn read_pdf(file: &str) -> Result<String, String> {
    match Document::load(file) {
        Ok(document) => match document.extract_text(&[1]) {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}
#[cfg(test)]
mod tests {
    use crate::chart_data::ChartData;

    #[test]
    fn test_extract() {
        let data = ChartData::read_data("/Users/john/Portfolio_Data/IQTrends/bluechips/1634249303__ETN_chart.pdf");
        println!("{:#?}", data);
    }

}
