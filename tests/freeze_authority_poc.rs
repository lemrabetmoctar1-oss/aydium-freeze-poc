use anchor_litesvm::AnchorLiteSVM;
use litesvm_utils::TestHelpers;

#[test]
fn freeze_authority_poc_step1() {
    let so_bytes = include_bytes!("../target/deploy/raydium_clmm.so");
    let program_id = solana_sdk::pubkey::Pubkey::new_unique();
    let mut ctx = AnchorLiteSVM::build_with_program(program_id, so_bytes);

    let attacker = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let mint = ctx.svm.create_token_mint(&attacker, 9).unwrap();

    println!("STEP 1 REAL PASS: program loaded, mint {} created", mint.pubkey());
}
