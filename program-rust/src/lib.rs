use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    /*instruction::{AccountMeta, Instruction},*/
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke,invoke_signed},
    rent::Rent,
    sysvar::Sysvar,
    system_instruction

};

use solana_sdk::{
  instruction::{AccountMeta, Instruction},
};

use spl_token::{
    solana_program::program_pack::Pack,
    state::{ Mint},
};

/*
use spl_token_client::{
    client::{ProgramBanksClient, ProgramBanksClientProcessTransaction, ProgramClient},
    token::Token,
};
*/

use {
    spl_associated_token_account::{
        get_associated_token_address,create_associated_token_account
    },
};

use std::convert::TryInto;


/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct StoreAssets {
    /// todo:change to allow bigger numbers
    pub price: u64,
}

entrypoint!(process_instruction);

pub mod grpc_mint_id {
    solana_sdk::declare_id!("GLoCBYmGqqUCZLy6XpmLUF9xAweUfe9R35pinscKzdZh");
}
pub mod contract_holder_mainid_id {
  solana_sdk::declare_id!("3d8ogqZX7aHmhpLkRqpEvnBNtCNBqVsVc3y7y83V78zL");
}
pub mod contract_holder_gcid_id {
  solana_sdk::declare_id!("Cfk7de1iQikNUMaayJSoiXZSz2v7XGhgkaMKtEArU1Aa");
}
pub mod initializers_main_account_id {
  solana_sdk::declare_id!("3d8ogqZX7aHmhpLkRqpEvnBNtCNBqVsVc3y7y83V78zL");
}

pub mod spl_associated_token_account_old_id{
  solana_sdk::declare_id!("3medvrcM8s3UnkoYqqV3RAURii1ysuT5oD7t8nmfgJma");
}

pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    _instruction_data: &[u8], 
) -> ProgramResult {
 
    let (&tag, rest) = _instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    let account_info_iter = &mut accounts.iter();

    //create an account at a PDA
	if tag == 4 {
        let initializers_main_account = next_account_info(account_info_iter)?;
		let pda_account_to_create = next_account_info(account_info_iter)?;
		let system_program_account = next_account_info(account_info_iter)?;
        let lamports:u64=946560;
        let total_space:u64=0;

        if *initializers_main_account.key != contract_holder_mainid_id::id(){
            msg!("Incorrect contract holder");
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

	//Adding a nft to the contract
	if tag == 2 {

		// Get the account to say hello to
        let initializers_main_account = next_account_info(account_info_iter)?;
		let nft_program_account = next_account_info(account_info_iter)?;

        if *initializers_main_account.key != initializers_main_account_id::id(){
            msg!("Incorrect initilizer");
            return Err(ProgramError::IncorrectProgramId);
        }
        if !initializers_main_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let price = rest.get(..8).and_then(|slice|slice.try_into().ok()).map(u64::from_le_bytes).ok_or(ProgramError::InvalidInstructionData)?;

	    //let nft_account = next_account_info(account_info_iter)?;
    	//let (nft_pubkey, bump_seed) = Pubkey::find_program_address(&[b"cJkQKU3ZJFXwfMnwdSoqgWvmDfBCU4yx"], program_id);
	 // Increment and store the number of times the account has been greeted
        let mut greeting_account = StoreAssets::try_from_slice(&nft_program_account.data.borrow())?;
        greeting_account.price = price;
        greeting_account.serialize(&mut &mut nft_program_account.data.borrow_mut()[..])?;
        msg!("counter is {}",greeting_account.price);
	return	Ok(())
	}
    
    
    //create asssociated token account
	if tag == 1 {
        let funder_info = next_account_info(account_info_iter)?;
        let associated_token_account_info = next_account_info(account_info_iter)?;
        let wallet_account_info = next_account_info(account_info_iter)?;
        let spl_token_mint_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;
        let spl_token_program_info = next_account_info(account_info_iter)?;
        let spl_token_program_id = spl_token_program_info.key;
        let rent_sysvar_info = next_account_info(account_info_iter)?;
        let associated_token_program_account_info = next_account_info(account_info_iter)?;

    	let (this_derived_program_address, bump_seed) = Pubkey::find_program_address(&[b"fundingaccount2"], program_id);


      //msg!("IS Signeer :{}",funder_info.is_signer);
      //&funder_info.is_signer = &true;

        let (&bump_seed, seed) = rest.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        msg!("Bump seed is :{}",bump_seed);
       // msg!("Seed is :{}",seed);
        //msg!("Program derived address is :{}",this_derived_program_address);

        let ix = create_associated_token_account(
              &funder_info.key,
             // &this_derived_program_address,
              &wallet_account_info.key,
              &spl_token_mint_info.key,
        );

        msg!("We at least got to the instruction");

        invoke_signed(
            &ix,
            &[
                funder_info.clone(),//question if the progam owns this address are we ok?
                associated_token_account_info.clone(), 
                wallet_account_info.clone(),
                spl_token_mint_info.clone(),
                system_program_info.clone(), 
                spl_token_program_info.clone(), 
                rent_sysvar_info.clone(), 
                associated_token_program_account_info.clone(),
            ],
             &[&[&seed, &[bump_seed]]]
       )?;

        return  Ok(())
    }

if tag == 3 {

    //to info
   let to_main_account = next_account_info(account_info_iter)?;
   let to_grepper_coin_associated_address = next_account_info(account_info_iter)?;
   let to_associated_nft_address = next_account_info(account_info_iter)?;
   
    //from info
   let grepper_nft_holder_account = next_account_info(account_info_iter)?;
   let grepper_nft_holder_account_associated_nft_address = next_account_info(account_info_iter)?;

    //meta
   let spl_token_program = next_account_info(account_info_iter)?;
   let derived_program_address = next_account_info(account_info_iter)?;
   
   let transfer_amount:u64=1;

   let (this_derived_program_address, bump_seed) = Pubkey::find_program_address(&[b"hello1"], program_id);

    //athority will be this program address... will that work??? let see
   let ix = spl_token::instruction::transfer(
        &spl_token::id(),//token program id
        grepper_nft_holder_account_associated_nft_address.key,//source pubkey
        to_associated_nft_address.key,//dest pubkey
        &this_derived_program_address,//athority
        &[],
        transfer_amount
    )?;

    for test_account in ix.accounts.iter(){
        msg!("Acount is {}",test_account.pubkey);
    }

        msg!("Acount 1 {}",grepper_nft_holder_account_associated_nft_address.key);
        msg!("Acount 2 {}",to_associated_nft_address.key);
        msg!("Acount 3 {}",derived_program_address.key);
    

 invoke_signed(
      &ix,
      &[
          //spl_token_program.clone(), 
          grepper_nft_holder_account_associated_nft_address.clone(), 
          to_associated_nft_address.clone(),
          derived_program_address.clone()
      ],
      &[&[b"hello1", &[bump_seed]]]
 );
}


/*
    msg!("tag {}",tag);

    let amount = rest.get(..8).and_then(|slice|slice.try_into().ok()).map(u64::from_le_bytes).ok_or(ProgramError::InvalidInstructionData)?;

    let grpc_mint = next_account_info(account_info_iter)?;

    //very important we hard code this to be correct because otherwise they can pass a different supply
    //let grpc_mint_key=Pubkey::from_str("GLoCBYmGqqUCZLy6XpmLUF9xAweUfe9R35pinscKzdZh").unwrap();
    if *grpc_mint.key != grpc_mint_id::id(){
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
    if *contract_holder_mainid.key != contract_holder_mainid_id::id(){
        msg!("Incorrect contract holder");
        return Err(ProgramError::IncorrectProgramId);
    }
    if *contract_holder_mainid.owner != *program_id{
        msg!("Incorrect contract holder");
        return Err(ProgramError::IncorrectProgramId);
    }

    let contract_holder_gcid = next_account_info(account_info_iter)?;
    if *contract_holder_gcid.key != contract_holder_gcid_id::id(){
        msg!("Incorrect contract holder");
        return Err(ProgramError::IncorrectProgramId);
    }
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

    */
    Ok(())
}
