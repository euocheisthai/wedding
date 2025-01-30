use chrono::{NaiveDate, Duration};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("ur doing it wrong: {} <start_date: YYYY-MM-DD>", args[0]);
        std::process::exit(1);
    }

    let start_date: NaiveDate = match NaiveDate::parse_from_str(&args[1], "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            eprintln!("Format date like this: YYYY-MM-DD.");
            std::process::exit(1);
        }
    };

    let doc_ready_date: NaiveDate = start_date + Duration::days(75);
    let doc_expiry_date: NaiveDate = doc_ready_date + Duration::days(180);
    let visa_start_date: NaiveDate = doc_ready_date;
    let visa_end_date: NaiveDate = visa_start_date + Duration::days(90);
    let latest_civil_date: NaiveDate = doc_expiry_date - Duration::days(60);
    let earliest_scheduling_date: NaiveDate = latest_civil_date - Duration::days(120);
    
    println!("MATT N LIDY SITTING IN A TREE...:");
    println!("1. Start gathering documents: {}", start_date);
    println!("2. Docs ready by (appox, worst case): {}", doc_ready_date);
    println!("3. Docs expire by (always 6 mon): {}", doc_expiry_date);
    println!("4. Apply for visa (takes 3 mon): {}", visa_start_date);
    println!("5. Visa should be ready by (appox, worst case): {}", visa_end_date);
    println!("6. Latest Civil Registry appt while docs are still valid: {}", latest_civil_date);
    println!("7. Civil Registry appt to schedule at (earliest possible, 4 mon in advance):   {}", earliest_scheduling_date);
}
