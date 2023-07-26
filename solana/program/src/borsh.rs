#![allow(clippy::arithmetic_side_effects)]
use {
    borsh::{
        maybestd::io::{Error, Write},
        schema::{BorshSchema, Declaration, Definition, Fields},
        BorshDeserialize, BorshSerialize,
    },
    std::collections::HashMap,
};

/// Get packed length for the given BorchSchema Declaration
fn get_declaration_packed_len(
    declaration: &str,
    definitions: &HashMap<Declaration, Definition>,
) -> usize {
    match definitions.get(declaration) {
        Some(Definition::Array { length, elements }) => {
            *length as usize * get_declaration_packed_len(elements, definitions)
        }
        Some(Definition::Enum { variants }) => {
            1 + variants
                .iter()
                .map(|(_, declaration)| get_declaration_packed_len(declaration, definitions))
                .max()
                .unwrap_or(0)
        }
        Some(Definition::Struct { fields }) => match fields {
            Fields::NamedFields(named_fields) => named_fields
                .iter()
                .map(|(_, declaration)| get_declaration_packed_len(declaration, definitions))
                .sum(),
            Fields::UnnamedFields(declarations) => declarations
                .iter()
                .map(|declaration| get_declaration_packed_len(declaration, definitions))
                .sum(),
            Fields::Empty => 0,
        },
        Some(Definition::Sequence {
            elements: _elements,
        }) => panic!("Missing support for Definition::Sequence"),
        Some(Definition::Tuple { elements }) => elements
            .iter()
            .map(|element| get_declaration_packed_len(element, definitions))
            .sum(),
        None => match declaration {
            "bool" | "u8" | "i8" => 1,
            "u16" | "i16" => 2,
            "u32" | "i32" => 4,
            "u64" | "i64" => 8,
            "u128" | "i128" => 16,
            "nil" => 0,
            _ => panic!("Missing primitive type: {}", declaration),
        },
    }
}

pub fn get_packed_len<S: BorshSchema>() -> usize {
    let schema_container = S::schema_container();
    get_declaration_packed_len(&schema_container.declaration, &schema_container.definitions)
}

pub fn try_from_slice_unchecked<T: BorshDeserialize>(data: &[u8]) -> Result<T, Error> {
    let mut data_mut = data;
    let result = T::deserialize(&mut data_mut)?;
    Ok(result)
}

/// Helper struct which to count how much data would be written during serialization
#[derive(Default)]
struct WriteCounter {
    count: usize,
}

impl Write for WriteCounter {
    fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
        let amount = data.len();
        self.count += amount;
        Ok(amount)
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

pub fn get_instance_packed_len<T: BorshSerialize>(instance: &T) -> Result<usize, Error> {
    let mut counter = WriteCounter::default();
    instance.serialize(&mut counter)?;
    Ok(counter.count)
}
