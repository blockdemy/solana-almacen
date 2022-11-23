use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize,BorshDeserialize, Debug)]
pub struct MathStuffSum{
    pub sum: u32,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: &[u8],
) -> ProgramResult{

    let accounts_iter = &mut accounts.iter();

    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id{

        msg!("Cuenta no tiene el correcto program_id");
        return Err(ProgramError::IncorrectProgramId);
    }
    
    msg!("Debug Cuenta");
    msg!("Cuenta Id: {}", account.key);
    msg!("Ejecutable {}",account.executable);
    msg!("Lamports {:#?}", account.lamports);

    msg!("Inicia Suma");

    let mut sum_stuff = MathStuffSum::try_from_slice(&account.data.borrow())?;
    sum_stuff.sum +=1;
    sum_stuff.serialize(&mut &mut account.data.borrow_mut()[..])?;
    
    msg!("Total: {}", sum_stuff.sum);

    Ok(())
}