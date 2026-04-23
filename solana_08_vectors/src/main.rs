use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap; //Llamamos a Hasmap //añade a Borsh para serializar y deserializar los datos de entrada y salida

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct StatsResult {
    pub median: u64,
    pub mode: u64,
}

fn process_instruction(input: &[u8]) -> Result<StatsResult, String> {
    let mut numbers: Vec<u64> = Vec::try_from_slice(input) //This process tries to convert the input bytes into a vector of u64 using Borsh deserialization.
        .map_err(|_| "Error al leer los datos")?;

    if numbers.is_empty() {
        return Err("El vector está vacío".to_string());
    }


    let ownrship = calculate_median(&mut numbers);

    let mut moda = HashMap::new();
    for &num in &numbers {
        *moda.entry(num).or_insert(0) += 1; // este cuenta la frecuencia de cada número en el vector
    }
    let mode = moda.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| num).unwrap_or(0);
    Ok(StatsResult { median: ownrship, mode })

}
// calcula la mediana
fn calculate_median(nums: &mut Vec<u64>) -> u64 {
    nums.sort_unstable();
    println!("Vector ordenado: {:?}", nums);
    let len = nums.len();
    if len % 2 == 0 {
        (nums[len / 2] as u128 + (nums[len / 2 - 1] as u128) / 2) as u64
    } else {
        nums[len / 2] as u64
    }
    
}

fn main() {
    let my_nums: Vec<u64> = vec![10, 5, 10, 3, 2];
    let encoded_input = borsh::to_vec(&my_nums).expect("Error al serializar los datos");

    
    match process_instruction(&encoded_input) {
        Ok(res) => println!("Resultado Solana: {:?}", res),
        Err(e) => println!("Error: {}", e),
    }
}
