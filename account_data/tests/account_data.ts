import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountData } from "../target/types/account_data";

declare('account data' , () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.AccountData as anchor.Program<AccountData>;

  //generate the keypair for the addressInfo account 
  const addressInfoAccount = new Keypair();


  it('Create the address info account ', async() => {
    console.log(`payer address : ${payer.publickey}`);
    console.log(`address_info account address : ${addressInfoAccount.publickey}`);

    // instruction INx data 
    const addressInfo = {
      name : 'ANURAG BAGRI',
      house_number : 136,
      street : 'near rims hospital',
      city : 'utopia'
    };
    
        await program.methods
      .createAddressInfo(addressInfo.name, addressInfo.house_number, addressInfo.street, addressInfo.city)
      .accounts({
        addressInfo: addressInfoAccount.publicKey,
        payer: payer.publicKey,
      })
      .signers([addressInfoAccount])
      .rpc();
  });
   it("Read the new account's data", async () => {
    const addressInfo = await program.account.addressInfo.fetch(addressInfoAccount.publicKey);
    console.log(`Name     : ${addressInfo.name}`);
    console.log(`House Num: ${addressInfo.house_number}`);
    console.log(`Street   : ${addressInfo.street}`);
    console.log(`City     : ${addressInfo.city}`);
  });
});