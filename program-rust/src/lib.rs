use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke,invoke_signed},
};

use {
    spl_associated_token_account::{
        create_associated_token_account
    },
};

use std::convert::TryInto;


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct StoreAssets {
    pub price: u64,
}

entrypoint!(process_instruction);

pub mod grepper_contributor_coin_mint_id {
    solana_sdk::declare_id!("6tAmokk5fqrjm4ho2JerziBsiV3hYzgJZnG6sFXZNXZs");
}
pub mod contract_holder_mainid_id {
  solana_sdk::declare_id!("3d8ogqZX7aHmhpLkRqpEvnBNtCNBqVsVc3y7y83V78zL");
}


pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    _instruction_data: &[u8], 
) -> ProgramResult {
 
    let (&tag, rest) = _instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    let account_info_iter = &mut accounts.iter();

    //buy a nft
    if tag == 1 {
        let funder_info = next_account_info(account_info_iter)?;
        let to_associated_nft_address = next_account_info(account_info_iter)?;
        let to_main_account = next_account_info(account_info_iter)?;
        let nft_mint_info = next_account_info(account_info_iter)?;

        let system_program_info = next_account_info(account_info_iter)?;
        if *system_program_info.key != solana_program::system_program::id(){
            return Err(ProgramError::InvalidAccountData);
        }

        let spl_token_program_info = next_account_info(account_info_iter)?;
        if *spl_token_program_info.key != spl_token::id(){
            return Err(ProgramError::InvalidAccountData);
        }

        let rent_sysvar_info = next_account_info(account_info_iter)?;
        if *rent_sysvar_info.key != solana_program::sysvar::rent::id(){
            return Err(ProgramError::InvalidAccountData);
        }

        let associated_token_program_account_info = next_account_info(account_info_iter)?;
        if *associated_token_program_account_info.key != spl_associated_token_account::id(){
            return Err(ProgramError::InvalidAccountData);
        }
    
          //to info
        let to_grepper_coin_associated_address = next_account_info(account_info_iter)?;
        let grepper_nft_holder_account = next_account_info(account_info_iter)?;
        let grepper_nft_holder_account_associated_nft_address = next_account_info(account_info_iter)?;


        let grepper_coin_mint = next_account_info(account_info_iter)?;
        if *grepper_coin_mint.key != grepper_contributor_coin_mint_id::id(){
               return Err(ProgramError::InvalidAccountData);
        }

        let transfer_amount:u64=1;
        let  asset = StoreAssets::try_from_slice(&grepper_nft_holder_account.data.borrow())?;
             asset.serialize(&mut &mut grepper_nft_holder_account.data.borrow_mut()[..])?;

        //first we burn/pay for NFT
        let ix = spl_token::instruction::burn(
            &spl_token::id(),
            to_grepper_coin_associated_address.key,
            &grepper_contributor_coin_mint_id::id(),
            to_main_account.key,
            &[],
            asset.price
        )?;

        invoke(
              &ix,
              &[to_grepper_coin_associated_address.clone(), grepper_coin_mint.clone(), to_main_account.clone(),spl_token_program_info.clone()]
        )?;

        let (&bump_seed, rest2) = rest.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        let (&bump_seed2, rest3) = rest2.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        let seed = &rest3[0..32];
        let seed2 = &rest3[32..64];

      if *to_associated_nft_address.owner == solana_program::system_program::id(){
          let ix = create_associated_token_account(
                &funder_info.key,
                &to_main_account.key,
                &nft_mint_info.key,
          );
          invoke_signed(
              &ix,
              &[
                  funder_info.clone(),
                  to_associated_nft_address.clone(), 
                  to_main_account.clone(),
                  nft_mint_info.clone(),
                  system_program_info.clone(), 
                  spl_token_program_info.clone(), 
                  rent_sysvar_info.clone(), 
                  associated_token_program_account_info.clone(),
              ],
               &[&[&seed, &[bump_seed]]]
         )?;
      }

     let ix = spl_token::instruction::transfer(
          &spl_token::id(),
          grepper_nft_holder_account_associated_nft_address.key,
          to_associated_nft_address.key,
          &grepper_nft_holder_account.key,
          &[],
          transfer_amount
      )?;

       invoke_signed(
            &ix,
            &[
                grepper_nft_holder_account_associated_nft_address.clone(), 
                to_associated_nft_address.clone(),
                grepper_nft_holder_account.clone()
            ],
           &[&[&seed2, &[bump_seed2]]]
       )?;

    return Ok(())
}


    //Adding a nft to the contract
if tag == 2 {

    let initializers_main_account = next_account_info(account_info_iter)?;
    let nft_program_account = next_account_info(account_info_iter)?;

    if *initializers_main_account.key != contract_holder_mainid_id::id(){
        return Err(ProgramError::IncorrectProgramId);
    }
    if !initializers_main_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let price = rest.get(..8).and_then(|slice|slice.try_into().ok()).map(u64::from_le_bytes).ok_or(ProgramError::InvalidInstructionData)?;

    
    let mut greeting_account = StoreAssets::try_from_slice(&nft_program_account.data.borrow())?;
    greeting_account.price = price;
    greeting_account.serialize(&mut &mut nft_program_account.data.borrow_mut()[..])?;
    return	Ok(())
}


//create an account to hold NFT at a PDA
if tag == 3 {
    let initializers_main_account = next_account_info(account_info_iter)?;
    let pda_account_to_create = next_account_info(account_info_iter)?;
    let system_program_account = next_account_info(account_info_iter)?;

    let lamports:u64=946560;
    let total_space:u64=8;

    if *initializers_main_account.key != contract_holder_mainid_id::id(){
        return Err(ProgramError::IncorrectProgramId);
    }
    if !initializers_main_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (&bump_seed, seed) = rest.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    let ix = solana_program::system_instruction::create_account(
        initializers_main_account.key,
        pda_account_to_create.key,
        lamports,
        total_space,
        program_id
    );

     invoke_signed(
          &ix,
          &[
              initializers_main_account.clone(),
              pda_account_to_create.clone(), 
              system_program_account.clone()
          ],
          &[&[&seed, &[bump_seed]]]
     )?;

    return	Ok(())
}

    

    return Err(ProgramError::InvalidInstructionData);
}
