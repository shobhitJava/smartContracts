use std::io::ErrorKind::InvalidData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub txt: String,
    pub counter: u32
}

const DUMMY_ID: &str = "0000";
pub fn get_init_account_message() -> GreetingAccount {
    GreetingAccount{ txt: String::from(DUMMY_ID), counter: 0 }
}
pub fn get_init_account_messages() -> GreetingAccount {
    msg!("in cretaion of accnt  passed as ");
    return get_init_account_message();
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
   // msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
   // msg!("before deserialize");
    let message= GreetingAccount::try_from_slice(instruction_data).map_err(|err|{
        msg!("Receiving msg as utf8 failed , {:?}",err);
        msg!("msg errror as {:?}", instruction_data);
        ProgramError::InvalidInstructionData
    })?;
    msg!("msg passed as {:?}", message);

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = match GreetingAccount::try_from_slice(&account.data.borrow_mut()){
        Ok(data) => {
            msg!("msg passed as {:?}", data); 
            data
            
        },
        Err(err) => {  
            if err.kind() == InvalidData {
                msg!("InvalidData so initializing account data");
                get_init_account_messages()
            } else {
                panic!("Unknown error decoding account data {:?}", err)
            }
      
           
        }
    };
 
    msg!("msg account before retrievd from account passed as {:?}", greeting_account);
    greeting_account.txt = message.txt;
    greeting_account.counter += 1;
    msg!("msg account retrievd from account passed as {:?}", greeting_account);
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

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
