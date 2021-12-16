use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke_signed,invoke},
};

use spl_token::{
    check_program_account,
    error::TokenError,
    instruction::{
        approve, transfer,initialize_account, initialize_mint, mint_to, revoke, set_authority, AuthorityType, burn
    },
    solana_program::program_pack::Pack,
    state::{Account, AccountState, Mint, Multisig},
};
use std::str::FromStr;
use std::convert::TryInto;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    _instruction_data: &[u8], 
) -> ProgramResult {
 
    let (&tag, rest) = _instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    let amount = rest.get(..8).and_then(|slice|slice.try_into().ok()).map(u64::from_le_bytes).ok_or(ProgramError::InvalidInstructionData)?;

    let account_info_iter = &mut accounts.iter();

    let grpc_mint = next_account_info(account_info_iter)?;

    //very important we hard code this to be correct because otherwise they canpass a different supply
    let grpc_mint_key=Pubkey::from_str("GLoCBYmGqqUCZLy6XpmLUF9xAweUfe9R35pinscKzdZh").unwrap();
    if *grpc_mint.key != grpc_mint_key{
        msg!("Incorrect Token Program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let main_token_program = next_account_info(account_info_iter)?;
    if *main_token_program.key != spl_token::id(){
          msg!("Incorrect token program");
          return Err(ProgramError::IncorrectProgramId);
    }

    let initializers_main_account = next_account_info(account_info_iter)?;
    if !initializers_main_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let initializers_token_account = next_account_info(account_info_iter)?;
    if *initializers_token_account.owner != *main_token_program.key{
        msg!("Incorrect owner of token account");
        return Err(ProgramError::IncorrectProgramId);
    }

    let contract_holder_mainid = next_account_info(account_info_iter)?;
    if *contract_holder_mainid.owner != *program_id{
        msg!("Incorrect contract holder");
        return Err(ProgramError::IncorrectProgramId);
    }

    let contract_holder_gcid = next_account_info(account_info_iter)?;
    if *contract_holder_gcid.owner != *main_token_program.key{
        msg!("Incorrect contract token holder");
        return Err(ProgramError::IncorrectProgramId);
    }



    let mint = Mint::unpack(&grpc_mint.data.borrow()).map_err(|_| ProgramError::InvalidInstructionData)?;

    let contractamportsBalance = contract_holder_mainid.lamports();
    let gcCoinsPerLamport =  mint.supply as f64 / contract_holder_mainid.lamports() as f64  ;
    let lamports_to_send_float =  amount as f64 / gcCoinsPerLamport as f64;
    let lamports_to_send = lamports_to_send_float as  u64;

    //so first we burn our spl token
    let ix = spl_token::instruction::burn(
        main_token_program.key,
        initializers_token_account.key,
        grpc_mint.key,
        initializers_main_account.key,
        &[&initializers_main_account.key],
        amount,
    )?;

     invoke(
          &ix,
          &[initializers_token_account.clone(), grpc_mint.clone(), initializers_main_account.clone(),main_token_program.clone()]
     );

    //send swapper the SOL
    **contract_holder_mainid.try_borrow_mut_lamports()? -= lamports_to_send;
    **initializers_main_account.try_borrow_mut_lamports()? += lamports_to_send;

    Ok(())
}
