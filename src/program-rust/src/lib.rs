use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

mod helper;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GetData {
    /// number of greetings
    pub client_seed: String,
    pub vec_len: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SetData {
    pub nonce: u32,
    pub random_number: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    // let get_account = next_account_info(accounts_iter)?;
    let set_account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if set_account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let get_data = GetData::try_from_slice(instruction_data).map_err(|err| {
        msg!("Get data failed {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    msg!("Get Data: {:?}", get_data);

    // Increment and store the number of times the account has been greeted
    let mut save_data = SetData::try_from_slice(&set_account.data.borrow())?;
    save_data.nonce += 1;
    // save_data.serialize(&mut &mut set_account.data.borrow_mut()[..])?;

    save_data.random_number = helper::generate_random_number(
        &get_data.client_seed, 
        save_data.nonce as u8, 
        get_data.vec_len
    );

    save_data.serialize(&mut &mut set_account.data.borrow_mut()[..])?;
    
    msg!("Nonce: {}", save_data.nonce);
    msg!("Random Number: {}", save_data.random_number);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
