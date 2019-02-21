#[derive(PartialEq, Debug)]
pub enum GridError {
    out_of_bounds,
    bot_exists_in_location,
    nothing_in_cell
}