fn main() {
// We are Assigning a variable calculate_weight_on_mar but we don't have to set type of variable in rust as it is smart enough to detect the type of variable which is f32(float)
    let mut mars_weight = calculate_weight_on_mars(100.0);
// In rust all variables are Immutable by default so for making it mutable there is a keyword Mut
    mars_weight = mars_weight * 1000.0;
    println!("Weight on mars: {}kg",mars_weight);
    
}
// ->  is the way os assigning return type of function which is f32
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711

}