#![allow(unused)]

use anyhow::Result;

mod parser;
mod potential;

use potential::UpetModel;

fn main() -> Result<()> {
    let file = "dump.out";
    let mut res = parser::read_dump(file)?;
    res
        .iter_mut()
        .for_each(|content| {
            content.atom_data.sort_by(|a, b| a.item_id.partial_cmp(&b.item_id).unwrap());
        });

    let model = UpetModel::load("./model.pt", false)?;
    Ok(())
}
