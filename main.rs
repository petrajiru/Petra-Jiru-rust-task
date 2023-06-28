/// Tietoevry's Entry Rust Assignment
///
/// Highest and lowest inflation rates in the history of the Czech and Slovak Republics & future values.
///
/// A rich factory owner looks back 30 years and asks whether it was wise to leave half of his monetary wealth in the Czech Republic and half in Slovak Republic after the division of Czechoslovakia.
/// 
/// You have received code that is missing some parts. Please fill them in so that the output of your code is the same as the output of the binary provided.
///
/// © Jakub Maly <jakub.maly@tietoevry.com>, 2023
///
// We want our hash map linked - we might ask you later why :)
// Note: You will need to cargo add this.
use linked_hash_map::LinkedHashMap;
// This helps to print large numbers nicely.
// Note: You will need to cargo add this as well.
use thousands::Separable;

// Data to use (1993 - 2022):
// Note: Do not change this part if you want to achieve the same results.
const STARTING_YEAR: usize = 1993;
const NUMBER_OF_YEARS: usize = 30;
const CZECH_REP_INF_RATES: [f64; NUMBER_OF_YEARS] = [
    0.28, 0.1, 0.091, 0.088, 0.085, 0.107, 0.021, 0.039, 0.047, 0.018, 0.001, 0.028, 0.019, 0.025,
    0.028, 0.063, 0.01, 0.015, 0.019, 0.033, 0.014, 0.004, 0.003, 0.007, 0.025, 0.021, 0.028,
    0.032, 0.038, 0.151,
];
const SLOVAK_REP_INF_RATES: [f64; NUMBER_OF_YEARS] = [
    0.232, 0.134, 0.099, 0.058, 0.061, 0.067, 0.106, 0.12, 0.073, 0.033, 0.085, 0.075, 0.027,
    0.045, 0.028, 0.046, 0.016, 0.01, 0.039, 0.036, 0.014, -0.001, -0.003, -0.005, 0.013, 0.025,
    0.027, 0.019, 0.032, 0.128,
];

/// Structure of linked hash map and several helper variables that will be used together as an inflation tracker.
#[derive(Default)]
pub struct InflationTracker {
    /// Name of the country
    country_name: String,
    /// Linked hash map for storing yearly value of inflation rate values
    yearly_val: LinkedHashMap<usize, f64>,
    /// Variable for storing the highest inflation rate year
    highest_year: Option<usize>,
    /// Variable for storing the highest inflation rate value
    highest_val: Option<f64>,
    /// Variable for storing the lowest inflation rate year
    lowest_year: Option<usize>,
    /// Variable for storing the lowest inflation rate value
    lowest_val: Option<f64>,
}

impl InflationTracker {
    /// Returns a new tracker with the given country name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the country
    ///
    /// # Returns
    ///
    /// * The new tracker
    ///
    pub fn new(name: &str) -> Self {
        let mut tracker: InflationTracker = InflationTracker::default();
        tracker.change_country_name(name);
        return tracker;
    }

    /// Inserts new value to the hash map and if needed updates the highest and the lowest variables as well.
    ///
    /// # Arguments
    ///
    /// * `year` - Year of measurement to be used as a key
    /// * `value` - Value of the measured inflation rate
    ///
    pub fn insert(&mut self, year: usize, value: f64) {
        self.yearly_val.insert(year, value);

        if self.yearly_val.len() == 1 {
            self.highest_val = Some(value);
            self.highest_year = Some(year);
            self.lowest_val = Some(value);
            self.lowest_year = Some(year);
        } else if self.highest_val < Some(value) {
            self.highest_val = Some(value);
            self.highest_year = Some(year);
        } else if self.lowest_val > Some(value) {
            self.lowest_val = Some(value);
            self.lowest_year = Some(year);
        }

    }

    /// Clears the hash map and sets all variables to default values.
    pub fn clear(&mut self) {
        self.country_name.clear();
        self.yearly_val.clear();
        self.highest_year = None;
        self.highest_val = None;
        self.lowest_year = None;
        self.lowest_val = None;
    }

    /// Changes the name of the country.
    ///
    /// # Arguments
    ///
    /// * `new_name` - New name of the country
    ///
    pub fn change_country_name(&mut self, new_name: &str) {
        self.country_name = new_name.to_string();
    }

    /// Returns the year and value of the maximum yearly inflation.
    ///
    /// # Returns
    ///
    /// * The year of the maximum yearly inflation
    /// * The value of the maximum yearly inflation
    ///
    pub fn get_max(&self) -> (usize, f64) {
        //return (self.highest_year.unwrap(), self.highest_val.unwrap());
        return self.yearly_val.iter().max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap()).map(|(y, v)| (*y, *v)).unwrap(); //bonus
    }

    /// Returns the year and value of the minimum yearly inflation.
    ///
    /// # Returns
    ///
    /// * The year of the minimum yearly inflation
    /// * The value of the minimum yearly inflation
    ///
    pub fn get_min(&self) -> (usize, f64) {
        //return (self.lowest_year.unwrap(), self.lowest_val.unwrap());
        return self.yearly_val.iter().min_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap()).map(|(y, v)| (*y, *v)).unwrap(); //bonus
    }

    /// Returns the future value of money for a given year range
    ///
    /// # Arguments
    ///
    /// * `money` - Original value
    /// * `start_year` - Year when money was worth 100%
    /// * `end_year` - Year for which we want to know the value
    ///
    ///
    /// # Returns
    ///
    /// * The future value of money for a given year
    ///
    /// Or displays a warning if:
    ///     No data is available
    ///     Start year is not in the data
    ///     End year -1 is not in the data
    ///     Start year is equal or greater than End year
    pub fn get_future_val(&self, money: f64, start_year: usize, end_year: usize) -> f64 {
        if start_year >= end_year {
            println!("Warning: Incorrect start year and end year!");
        }

        let mut current_money: f64 = money;
        let mut current_year: usize = start_year;
        while current_year < end_year {
            let rate = self.yearly_val.get(&current_year);
            if rate.is_none(){
                println!("Warning: Incorrect data!");
            }
            current_money *= 1.00 - rate.unwrap();
            current_year += 1;
        }

        return current_money;
    }

    /// Prints stored data as:
    ///
    /// Data for country_name are:
    /// year_0 | rate_0
    /// year_N | rate_N
    ///
    /// Or displays a warning if no data is available to print.
    pub fn print_data(&self) {
        if self.yearly_val.len() == 0 {
            println!("Warning: Found no data to print!");
            return
        }

        for (year, value) in self.yearly_val.iter() {
            println!("{} | {}%", year, format!("{:.2}", value * 100.0));
        }
    }
}

/// Rounds a number to two decimal points.
///
/// # Arguments
///
/// * `num` - Number to round
///
/// # Returns
///
/// * The rounded number
///
fn round2(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}

fn main() {
    // Initialize the tracker.

    // Insert the data for the first country.
    let mut tracker: InflationTracker = InflationTracker::new("Czech Republic");
    let mut year: usize = STARTING_YEAR;
    for value in CZECH_REP_INF_RATES {
        tracker.insert(year, value);
        year += 1;
    }

    // Print inserted data.
    println!("Data for Czech Republic:");
    tracker.print_data();

    // Print the maximum value of inflation rate and the year for the first country in percentage.
    println!("Maximum was {}: {}%", tracker.highest_year.unwrap(), format!("{:.2}", 100.0 * tracker.highest_val.unwrap()));

    // Print the minimum value of inflation rate and the year for the first country in percentage.
    println!("Minimum was {}: {}%", tracker.lowest_year.unwrap(), format!("{:.2}", 100.0 * tracker.lowest_val.unwrap()));

    // Print the result of saving 5,000,000 in cash for 30 years.
    // Do not forget to save this for later evaluation.
    let saved_money: f64 = 5000000.0;
    let czech_val: f64 = tracker.get_future_val(saved_money,STARTING_YEAR, STARTING_YEAR + NUMBER_OF_YEARS);
    println!("Saving {} for 30 years in 1993 with no interest rate would mean having {} in 2023", saved_money.separate_with_commas(), round2(czech_val).separate_with_commas());
    println!();

    // Clear the tracker.
    // Do not create a new one! reuse reuse reuse :)
    tracker.clear();

    // Check that it was cleared - print_data.
    tracker.print_data();
    println!();

    // Insert the data for the second country.
    year = STARTING_YEAR;
    for value in SLOVAK_REP_INF_RATES {
        tracker.insert(year, value);
        year += 1;
    }

    // Do not forget to change the name!
    tracker.change_country_name("Slovak Republic");

    // Print inserted data.
    println!("Data for Slovak Republic:");
    tracker.print_data();

    // Print the maximum value of inflation rate and the year for the first country in percentage.
    println!("Maximum was {}: {}%", tracker.highest_year.unwrap(), format!("{:.2}", 100.0 * tracker.highest_val.unwrap()));

    // Print the minimum value of inflation rate and the year for the first country in percentage.
    println!("Minimum was {}: {}%", tracker.lowest_year.unwrap(), format!("{:.2}", 100.0 * tracker.lowest_val.unwrap()));

    // Print the result of saving 5,000,000 in cash for 30 years.
    // Do not forget to save this for later evaluation.
    let slovak_val: f64 = tracker.get_future_val(saved_money,STARTING_YEAR, STARTING_YEAR + NUMBER_OF_YEARS);
    println!("Saving {} for 30 years in 1993 with no interest rate would mean having {} in 2023", saved_money.separate_with_commas(), round2(slovak_val).separate_with_commas());
    println!();

    // Print which country has had higher inflation in the last 30 years.
    if czech_val > slovak_val{
        println!("Slovak Republic had a higher inflation rate than Czech Republic!");
    } else {
        println!("Czech Republic had a higher inflation rate than Slovak Republic!");
    }
    
    

}

// BONUS POINTS:

// Get the maximum and minimum without using any helper variable.
// Hint: Take a look at `max_by()` and `min_by()` hash map functions.

// Move implementation to a separate module.
// Hint: There is reason for `pub` keywords.
