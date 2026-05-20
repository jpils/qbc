use std::{time, usize};
use std::{fs::File, path::Path};
use std::io::{BufReader, Lines, BufRead};
use anyhow::{Context, Result};
use nalgebra::{Matrix3, Vector3};
use strum_macros::EnumString;

pub type Matrix = Matrix3<f64>;
pub type Vector = Vector3<f64>;

#[derive(Debug, Default, EnumString)]
pub enum Element {
    Sr, Ti, O, H, Ag, I, 
    #[default]
    Unknown
}

#[derive(Default, Debug)]
pub struct AtomData {
    item_id: usize,
    atom_id: usize,
    element: Element,
    position: Vector
}

pub struct FileContent {
    pub timestep: usize,
    pub atom_count: usize,
    pub box_bounds: Matrix,
    pub atom_data: Vec<AtomData>
}

pub fn read_dump<P: AsRef<Path>>(filepath: P) -> Result<Vec<FileContent>> {
    let filepath = filepath.as_ref();

    let file = File::open(filepath)?;
    let mut lines = BufReader::new(file).lines();
    
    let _ = lines
        .next()
        .transpose()?
        .context("File is empty");

    let mut file_contents = Vec::new();
    loop {
        let header = read_header(&mut lines)?;
        let atom_data = read_atoms(header.atom_count, &mut lines)?;

        let content = FileContent {
            timestep: header.timestep,
            atom_count: header.atom_count,
            box_bounds: header.box_bounds,
            atom_data: atom_data
        };
        
        file_contents.push(content);

        if lines.next().transpose()?.is_none() {
            break;
        }
    }

    Ok(file_contents)
}

struct Header {
    timestep: usize,
    atom_count: usize,
    box_bounds: Matrix
}

fn read_header(lines: &mut Lines<BufReader<File>>) -> Result<Header> {
    let timestep = lines
        .next()
        .transpose()?
        .context("Timestep missing")?
        .trim()
        .parse()
        .context("Failed to parse timestep as usize");

    lines.next();

    let atom_count = lines
        .next()
        .transpose()?
        .context("Number of atoms missing")?
        .trim()
        .parse()
        .context("Failed to parse atom_count as usize");

    lines.next();

    let mut box_bounds: Vec<Vector> = Vec::with_capacity(3);
    for _ in 0..3 {
        let vec: Vec<f64> = lines
            .next()
            .transpose()?
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<f64>().map_err(Into::into))
            .collect::<Result<Vec<_>>>()?;

        box_bounds.push(Vector::from_iterator(vec));
    }
        
    let box_bounds_t = box_bounds
        .iter()
        .map(nalgebra::Matrix::transpose)
        .collect::<Vec<_>>();

    let header = Header {
        timestep: timestep?,
        atom_count: atom_count?,
        box_bounds: Matrix::from_rows(&box_bounds_t)
    };

    Ok(header)
}

fn read_atoms(n_atoms: usize, lines: &mut Lines<BufReader<File>>) -> Result<Vec<AtomData>> {
    let mut data = Vec::with_capacity(n_atoms);

    lines.next();
    for _ in 0..n_atoms {
        let line = lines
            .next()
            .transpose()?
            .context("Expected Atom data")?;
        let s = line
            .split_whitespace()
            .collect::<Vec<_>>();

        let item_id = s[0].parse::<usize>()?;
        let atom_id = s[1].parse::<usize>()?; 
        let element = s[2].parse::<Element>()?;
        let position = s[3..]
            .iter()
            .map(|s| s.parse::<f64>().map_err(Into::into))
            .collect::<Result<Vec<_>>>()?;
       let position = Vector::from_vec(position);

       let atom_data = AtomData {
           item_id, 
           atom_id, 
           element, 
           position 
       };

        data.push(atom_data);
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};

    const REAL_DUMP_DATA: &str = include_str!("dump.out");

    use tempfile::NamedTempFile; 

    fn open_test_file() -> Result<(std::io::Lines<BufReader<File>>, NamedTempFile)> {
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(REAL_DUMP_DATA.as_bytes())?;
        
        temp_file.flush()?; 
        
        let file = File::open(temp_file.path())?;
        let lines = BufReader::new(file).lines();
        
        Ok((lines, temp_file))
    }

    #[test]
    fn test_read_header() -> Result<()> {
        let (mut lines, _handle) = open_test_file()?;
        
        lines.next().transpose()?; 

        let header = read_header(&mut lines)?;

        assert_eq!(header.timestep, 0);
        assert_eq!(header.atom_count, 480);

        assert!((header.box_bounds[(0, 0)] - 0.0).abs() < 1e-6);
        assert!((header.box_bounds[(0, 1)] - 32.69048).abs() < 1e-6);

        Ok(())
    }

    #[test]
    fn test_read_atoms() -> Result<()> {
        let (mut lines, _handle) = open_test_file()?;

        for _ in 0..8 {
            lines.next().transpose()?;
        }
        let atoms = read_atoms(5, &mut lines)?;

        assert_eq!(atoms.len(), 5);
        
        let atom_0 = &atoms[0];
        assert_eq!(atom_0.item_id, 260);
        assert_eq!(atom_0.atom_id, 3);
        assert!(matches!(atom_0.element, Element::O));
        
        assert!((atom_0.position.x - 6.81024).abs() < 1e-5);
        assert!((atom_0.position.y - 5.92816).abs() < 1e-5);
        assert!((atom_0.position.z - 7.66255).abs() < 1e-5);

        Ok(())
    }

    #[test]
    fn test_read_dump() -> Result<()> {
        let (_lines, handle) = open_test_file()?;
        let mut contents = read_dump(handle.path())?;
        let frame = &mut contents[0];

        assert_eq!(frame.atom_data.len(), 480);
        Ok(())
    }
}
