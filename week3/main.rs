// Define a struct called FilterCondition, which will hold a target_value.
struct FilterCondition {
    target_value: String, // Change the type to the desired one (e.g., String)
}

// Implementation block for FilterCondition.
impl FilterCondition {
    // Define a method is_match that checks if a given item matches the target_value.
    fn is_match(&self, item: &String) -> bool {
        item == &self.target_value
    }
}

// Define a custom_filter function that filters a collection based on a condition.
fn custom_filter(collection: Vec<String>, condition: &FilterCondition) -> Vec<String> {
    // Use iterator's filter method to keep items that match the condition.
    collection.into_iter().filter(|item| condition.is_match(item)).collect()
}

fn main() {
    // Create a collection of Strings.
    let collection = vec![String::from("a"), String::from("b"), String::from("c")];

    // Create a FilterCondition instance with a target_value of "a".
    let filter_condition = FilterCondition {
        target_value: String::from("b"),
    };

    // Use custom_filter to filter the collection based on the condition.
    let filtered_data = custom_filter(collection, &filter_condition);

    // Print the filtered data.
    println!("Filtered Data: {:?}", filtered_data);
}
