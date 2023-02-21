//Objects And Classes Assignment - Kitchen Locator App
//Max Edward | 2/21/23

//Creates Appliance struct, which stores the name and the location in strings.
struct Appliance {
    name: String,
    location: String,
}

//Implementation of Appliance Struct
impl Appliance {
    //Creates a new "appliance" instance, which stores the name and location per appliance.
    fn new(name: String, location: String) -> Appliance {
        Appliance {
            name: name,
            location: location,
        }
    }

    //Method for taking location reference and storing it as a string. 
    //This in particular is creating a new string with the same contents as the original "location", 
    //this is before a new location is manually defined.
    fn locate(&self) -> String {
        self.location.clone()
    }
}

//Creates Kitchen struct, which stores the appliances vector of Appliance struct
struct Kitchen {
    appliances: Vec<Appliance>,
}

//Implementation of Kitchen Struct
impl Kitchen {
    //Creates and returns a new "kitchen" instance. This allows for the new function to be called.
    fn new() -> Kitchen {
        Kitchen {
            appliances: Vec::new(),
        }
    }

    //This creates a new appliance with the same contents as the original "appliance".
    //Allows for an appliance to be added to the kitchen vector, 
    //this is before a new appliance name is defined
    fn add_appliance(&mut self, appliance: Appliance) {
        self.appliances.push(appliance);
    }

    //This function searches for an appliance in the kitchen vector by its name. 
    //For each appliance, it verifies if the name matches the name argument. 
    //If so, then it returns using the Some variant. if not, then it returns using the None variant.
    fn find_appliance(&self, name: &str) -> Option<&Appliance> {
        for appliance in &self.appliances {
            if appliance.name == name {
                return Some(appliance);
            }
        }
        None
    }
}


//Main function
fn main() {

    //Creates variable "kitchen" tied to the kitchen struct.
    let mut kitchen = Kitchen::new();

    //Creates a new appliance called "microwave"
    kitchen.add_appliance(Appliance::new(
        "Microwave".to_string(),
        "Counter".to_string(),
    ));

    //Creates a new appliance called "oven"
    kitchen.add_appliance(Appliance::new(
        "Oven".to_string(), 
        "North Wall".to_string()
    ));

    //Createss a new appliance called "refrigerator"
    kitchen.add_appliance(Appliance::new(
        "Refrigerator".to_string(),
        "Corner".to_string(),
    ));

    //Creates a new appliance called "blender"
    kitchen.add_appliance(Appliance::new(
        "Blender".to_string(), 
        "Island".to_string()
    ));

    //Creates a new appliance called "dishwasher"
    kitchen.add_appliance(Appliance::new(
        "Dishwasher".to_string(),
        "South Wall".to_string(),
    ));

    //Defines variable, count, that is able to count the number of appliances within the kitchen struct.
    let count = kitchen.appliances.len();

    //Print out lines to the user.
    println!("\nWelcome to the Kitchen Locator.\n");
    println!("Locating Appliances...\n");

    //Display number of appliances found.
    println!("Found {} Appliances:\n", count);

    //Define each appliance to a new variable.
    let _appliance_m = kitchen.find_appliance("Microwave").unwrap();
    let _appliance_o = kitchen.find_appliance("Oven").unwrap();
    let _appliance_r = kitchen.find_appliance("Refrigerator").unwrap();
    let _appliance_b = kitchen.find_appliance("Blender").unwrap();
    let _appliance_d = kitchen.find_appliance("Dishwasher").unwrap();

    //Read out each appliance variable name and location.
    println!(
        "The {} is located on the {}.\n",
        _appliance_m.name,
        _appliance_m.locate()
    );
    println!(
        "The {} is located on the {}.\n",
        _appliance_o.name,
        _appliance_o.locate()
    );
    println!(
        "The {} is located on the {}.\n",
        _appliance_r.name,
        _appliance_r.locate()
    );
    println!(
        "The {} is located on the {}.\n",
        _appliance_b.name,
        _appliance_b.locate()
    );
    println!(
        "The {} is located on the {}.\n",
        _appliance_d.name,
        _appliance_d.locate()
    );
}
