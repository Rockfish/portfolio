#![allow(dead_code)]

use crate::chart_data::{ChartData, Flag};
use lopdf::Document;
use std::fs::File;
use std::io::{Read, Write};

pub fn read_chart_data(filename: &str) -> Result<ChartData, String> {
    let mut text: String = String::new();

    if filename.ends_with("pdf") {
        text = read_pdf(filename).unwrap();
    } else {
        let mut file = File::open(filename).unwrap();
        file.read_to_string(&mut text).unwrap();
    }

    if text.contains("Unimplemented") {
        let mut output = File::create(filename.replace("pdf", "_error.txt")).unwrap();
        output.write_all(text.as_bytes()).unwrap();
        let msg = format!("Error reading file: {}", filename);
        return Err(msg);
    }

    match extract_data(text) {
        Ok(mut data) => {
            let (symbol, chart_file) = get_symbol_and_chart(filename);
            data.symbol = symbol;
            data.chart = chart_file;
            data.update_date = chrono::offset::Local::now().to_string();
            Ok(data)
        }
        Err(e) => Err(e),
    }
}

fn get_symbol_and_chart(file: &str) -> (String, String) {
    let filename = file.replace("IQT_", "").replace("_good.txt", "pdf");
    let start = filename.find("__").unwrap();
    let end = filename.find("_chart").unwrap();
    let symbol = filename[start + 2..end].to_uppercase().to_string();
    let start = filename.rfind('/').unwrap();
    let chart_file = filename[start + 1..].to_string();
    (symbol, chart_file)
}

fn extract_data(text: String) -> Result<ChartData, String> {
    let lines: Vec<String> = text.split('\n').map(|t| t.to_string()).collect();
    let mut data = ChartData::default();
    let mut flag = Flag::None;

    for line in lines {
        if line.is_empty() || line.starts_with("Investment") || line.starts_with("http") || line.starts_with("INVESTMENT") {
            continue;
        }
        if let Some(stripped) = line.strip_prefix("Quality Rank:") {
            data.quality_rank = stripped.trim().to_string();
        }
        if let Some(stripped) = line.strip_prefix("Shares Outstg (M):") {
            data.shares_outstanding = stripped.trim().to_string();
        }
        if let Some(stripped) = line.strip_prefix("Shares (in mil):") {
            data.shares_outstanding = stripped.trim().to_string();
        }
        if let Some(stripped) = line.strip_prefix("Com Shs Outstg in Mil:") {
            data.shares_outstanding = stripped.trim().to_string();
        }
        if let Some(stripped) = line.strip_prefix("Inst Own:") {
            data.institution_own = stripped.trim().replace('%', "").to_string();
        }
        if let Some(stripped) = line.strip_prefix("Div Paid Since:") {
            data.div_paid_since = stripped.trim().to_string();
        }
        if let Some(stripped) = line.strip_prefix("Profit Margin:") {
            data.profit_margin = stripped.trim().replace('%', "").to_string();
        }
        if let Some(stripped) = line.strip_prefix("TTM Earnings:") {
            data.ttm_earnings = stripped.trim().replace('$', "").to_string();
        }
        if let Some(stripped) = line.strip_prefix("P/E Ratio:") {
            data.pe_ratio = stripped.trim().to_string();
        }
        if let Some(stripped) = line.strip_prefix("Book Value:") {
            data.book_value = stripped.trim().to_string();
        }
        if let Some(stripped) = line.strip_prefix("Div Payout:") {
            data.div_payout = stripped.trim().replace('%', "").to_string();
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
        if let Some(stripped) = line.strip_prefix("Price:") {
            let price = stripped.trim().to_string();
            match flag {
                Flag::Current => data.current_price = price,
                Flag::Overvalue => data.overvalue_price = price,
                Flag::Undervalue => data.undervalue_price = price,
                _ => {}
            }
        }
        if let Some(stripped) = line.strip_prefix("Yield:") {
            let value = stripped.trim().replace('%', "").to_string();
            match flag {
                Flag::Current => data.current_yield = value,
                Flag::Overvalue => data.overvalue_yield = value,
                Flag::Undervalue => data.undervalue_yield = value,
                _ => {}
            }
        }
        if let Some(stripped) = line.strip_prefix("Pts Up:") {
            data.overvalue_pts_up = stripped.trim().to_string();
        }
        if let Some(stripped) = line.strip_prefix("% Up:") {
            data.overvalue_percent_up = stripped.trim().replace('%', "").to_string();
        }
        if let Some(stripped) = line.strip_prefix("Pts Dn:") {
            data.undervalue_pts_dn = stripped.trim().to_string();
        }
        if let Some(stripped) = line.strip_prefix("Dn:") {
            data.undervalue_percent_dn = stripped.trim().replace('%', "").to_string();
        }
        if let Some(stripped) = line.strip_prefix("% Dn:") {
            data.undervalue_percent_dn = stripped.trim().replace('%', "").to_string();
        }
    }

    if data.current_price.is_empty() {
        return Err("Error parsing data".to_string());
    }

    Ok(data)
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
    use crate::chart_data::extract_data::*;
    use std::collections::HashSet;
    use std::fs::{read_dir, File};
    use std::io::{Read, Write};

    #[test]
    fn test_extract() {
        let data = read_chart_data("/Users/john/Portfolio_Data/IQTrends/bluechips/1634249303__ETN_chart.pdf");
        println!("{:#?}", data);
    }

    #[test]
    fn test_extract_old_chart() {
        let data = read_chart_data("/Users/john/Portfolio_Data/IQTrends/bluechips/1537033430__AEP_chart.pdf");
        println!("{:#?}", data);
    }

    #[test]
    fn test_extract_all_files() {
        let fixed = vec![
            "1459466096__IQT_dte_chart._good.txt",
            "1459466122__IQT_hei_chart._good.txt",
            "1459466150__IQT_lufk_chart._good.txt",
            "1459466170__IQT_noc_chart._good.txt",
            "1459466232__IQT_tr_chart._good.txt",
            "1459466241__IQT_ul_chart._good.txt",
            "1459466260__IQT_wst_chart._good.txt",
            "1488334235__IQT_JWA_chart2._good.txt",
            "1490922505__IQT_MGEE_chart._good.txt",
            "1501606137__AOS_chart._good.txt",
            "1508177929__GE_chart._good.txt",
            "1509446011__ENB_chart._good.txt",
            "1510666199__SCG_chart._good.txt",
            "1513852266__CW_chart._good.txt",
            "1513871028__XEL_chart._good.txt",
            "1520531040__FRT_chart._good.txt",
            "1522441165__OMI_chart2._good.txt",
            "1529008329__IR_chart2._good.txt",
            "1530224312__TAP_chart2._good.txt",
            "1537033430__AEP_chart._good.txt",
            "1541026597__WSO_chart2._good.txt",
            "1543877977__ESS_chart2._good.txt",
            "1680281344__T_chart._good.txt",
            "1680281432__OXY_chart._good.txt",
            "1680281491__SLB_chart._good.txt",
            "1680281578__SON_chart._good.txt",
            "1680281707__TRN_chart._good.txt",
            "1680281810__VSEC_chart._good.txt",
        ];

        let mut fixed_files = HashSet::new();

        for f in &fixed {
            let filename = f.replace("._good.txt", ".pdf");
            fixed_files.insert(filename);
        }

        let pdfs: Vec<String> = read_dir("/Users/john/Portfolio_Data/IQTrends/bluechips/")
            .unwrap()
            .map(|x| x.unwrap().file_name().into_string().unwrap())
            .filter(|f| f.ends_with("pdf"))
            .filter(|f| !fixed_files.contains(f))
            .collect();

        let mut chart_data: Vec<ChartData> = vec![];

        for filename in fixed {
            let full_path = format!("{}{}", "/Users/john/Portfolio_Data/IQTrends/bluechips/", filename);
            match read_chart_data(&full_path) {
                Ok(data) => chart_data.push(data),
                Err(e) => println!("{}", e),
            }
        }

        for filename in pdfs {
            let full_path = format!("{}{}", "/Users/john/Portfolio_Data/IQTrends/bluechips/", filename);
            match read_chart_data(&full_path) {
                Ok(data) => chart_data.push(data),
                Err(e) => println!("{}", e),
            }
        }

        chart_data.sort_by(|a, b| a.symbol.cmp(&b.symbol));

        // let data_str = ron::to_string(&chart_data).unwrap();
        let data = serde_json::to_string_pretty(&chart_data).unwrap();
        let mut output = File::create("chart_data.json").unwrap();
        output.write(data.as_bytes()).unwrap();
    }

    #[test]
    fn test_reading_chart_data() {
        let mut file = File::open("chart_data.json").unwrap();

        let mut data: String = String::new();
        file.read_to_string(&mut data).unwrap();

        let chart_data = serde_json::from_str::<Vec<ChartData>>(&data).unwrap();

        for chart in chart_data {
            println!("{:?}", chart);
        }
    }
}
