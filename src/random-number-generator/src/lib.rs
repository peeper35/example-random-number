use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    log::sol_log_compute_units
};

mod helper;

/// Struct which will be used to get data from the client
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GetData {
    pub client_seed: String,
    pub vec_len: u32,
}

// Struct which will be used to serialize and store data into the account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SetData {
    pub nonce: u32,
    pub random_number: u32,
}

// Program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8], 
) -> ProgramResult {
    msg!("Random Number Rust program entrypoint");

    // An array of accounts which will be passed to the program
    let accounts_iter = &mut accounts.iter();

    // We will need only one account to store the data
    let set_account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if set_account.owner != program_id {
        msg!("Account Owner check failed!");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get the byte array instruction data and deserialize it into the GetData Struct
    // client_seed and vec_len will be passed to program to generate the random number
    let get_data = GetData::try_from_slice(instruction_data).map_err(|err| {
        msg!("Get data failed {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    // logging of the deserialized data
    msg!("Get Data: {:?}", get_data);

    // nonce will start from 0
    // and it will be incremented everytime any user will ping the program for random number
    let mut save_data = SetData::try_from_slice(&set_account.data.borrow())?;
    save_data.nonce += 1;
 
    // generate the random number and then store it into the struct
    save_data.random_number = helper::generate_random_number(
        &get_data.client_seed,
        save_data.nonce as u8,
        get_data.vec_len,
    );

    // the struct/data will serialized into the account
    save_data.serialize(&mut &mut set_account.data.borrow_mut()[..])?;

    sol_log_compute_units();
    
    // some more logging
    msg!("Nonce: {}", save_data.nonce);
    msg!("Random Number: {}", save_data.random_number);

    Ok(())
}