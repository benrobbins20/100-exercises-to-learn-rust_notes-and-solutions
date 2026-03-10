// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.

// struct defines the type 
pub struct Order {
    product_name: String, 
    quantity: u32,
    unit_price: u32,
}

// impl creates functionality for the type
// inherent method implementation
impl Order {
    // constructor for a new Order instance
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        // call validation functions, new() will panic if methods fail/ params wrong
        validate_name(&product_name);
        validate_quantity(&quantity);
        validate_price(&unit_price);
        // then return the instance
        Order { product_name, quantity, unit_price }
    }

    // getters, name = order1.product_name()
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    // setters, borrow mutable reference to order instance (&mut self)
    pub fn set_product_name(&mut self, product_name: String) {
        validate_name(&product_name);
        self.product_name = product_name;
    }
    pub fn set_quantity(&mut self, quantity: u32) {
        validate_quantity(&quantity);
        self.quantity = quantity;
    }
    pub fn set_unit_price(&mut self, unit_price: u32) {
        validate_price(&unit_price);
        self.unit_price = unit_price;
    }

    // return the total qty*price
    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}


fn validate_name(name: &String) {
    if name.is_empty() {
        panic!("Name is empty");
    }
    if name.len() > 300 {
        panic!("Name too long");
    }
}

// compare to reference to u32 0 anonymous variable on stack 
fn validate_quantity(quantity: &u32) {
    if quantity == &0 {
        panic!("Qty must be > 0");
    }
}

fn validate_price(price: &u32) {
    if price == &0 {
        panic!("Price must be > 0");
    }
}
