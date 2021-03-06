use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

solana_program::declare_id!("BpfProgram1111111111111111111111111111111111");

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        0 => vector_init_macro(),
        1 => vector_init_no_cap_push(),
        2 => vector_init_cap_push(),
        3 => for_loop_no_iter_no_ref_assign_simple_type(),
        4 => for_loop_iter_no_ref_assign_simple_type(),
        5 => for_loop_no_iter_no_ref_complex_type_assign(),
        6 => for_loop_no_iter_ref_complex_type_assign(),
        7 => for_loop_no_iter_ref_assign_simple_type(),
        8 => for_loop_iter_ref_assign_simple_type(),
        9 => pass_pubkey_no_ref(&accounts[0]),
        10 => pass_pubkey_ref(&accounts[0]),
        11 => pubkey_default_multicall(&accounts[0]),
        12 => pubkey_default_variable(&accounts[0]),
        13 => print_string_format_args(),
        14 => print_string_format(),
        _ => (),
    };
    Ok(())
}


fn vector_init_macro() {
    let a = vec![1, 2, 3, 4, 5];
}

fn vector_init_no_cap_push() {
    let mut a = Vec::new();
    a.push(1_i32);
    a.push(2_i32);
    a.push(3_i32);
    a.push(4_i32);
    a.push(5_i32);
}

fn vector_init_cap_push() {
    let mut a = Vec::with_capacity(5);
    a.push(1_i32);
    a.push(2_i32);
    a.push(3_i32);
    a.push(4_i32);
    a.push(5_i32);
}

fn for_loop_no_iter_no_ref_assign_simple_type() {
    let a = vec![1, 2, 3, 4, 5];
    for i in 0..a.len() {
        let _b = a[i];
    }    
}

fn for_loop_no_iter_ref_assign_simple_type() {
    let a = vec![1, 2, 3, 4, 5];
    for i in 0..a.len() {
        let _b = &a[i];
    }    
}


fn for_loop_iter_no_ref_assign_simple_type() {
    let a = vec![1, 2, 3, 4, 5];
    for i in a.iter() {
        let _b = *i;
    }    
}

fn for_loop_iter_ref_assign_simple_type() {
    let a = vec![1, 2, 3, 4, 5];
    for i in a.iter() {
        let _b = i;
    }    
}


fn for_loop_no_iter_no_ref_complex_type_assign() {
    let a = vec![
        (0, 1, "abc".to_string()),
        (0, 1, "abcd".to_string()),
        (0, 1, "abcde".to_string()),
        (0, 1, "abcdef".to_string()),
    ];
    for i in 0..a.len() {
        let _b = a[i].clone();
    }
}

fn for_loop_no_iter_ref_complex_type_assign() {
    let a = vec![
        (0, 1, "abc".to_string()),
        (0, 1, "abcd".to_string()),
        (0, 1, "abcde".to_string()),
        (0, 1, "abcdef".to_string()),
    ];
    for i in a.iter() {
        let _b = i.clone();
    }
}

fn pass_pubkey_no_ref(acct: &AccountInfo) {
    pubkey_pass_no_ref(*acct.key)
}

fn pass_pubkey_ref(acct: &AccountInfo) {
    pubkey_pass_ref(acct.key)
}


fn pubkey_default_multicall(acct: &AccountInfo) {
    assert!(acct.key != &Pubkey::default());
    assert!(acct.key != &Pubkey::default());
}

fn pubkey_default_variable(acct: &AccountInfo) {
    let default_key = Pubkey::default();
    assert!(acct.key != &default_key);
    assert!(acct.key != &default_key);
}

fn pubkey_pass_no_ref(key: Pubkey) {
    let a = key;
    assert!(a != Pubkey::default());
}


fn pubkey_pass_ref(key: &Pubkey) {
    let a = key;
    assert!(a != &Pubkey::default());
}

fn print_string_format_args() {
    msg!("{}", format_args!(
        "this {} a {} string {} print {}",
        "is", "long", "to", "yoyoyoyo"
    ));
}

fn print_string_format() {
    msg!("{}", format!(
        "this {} a {} string {} print {}",
        "is", "long", "to", "yoyoyoyo"
    ));   
}



#[cfg(test)] 
mod test {
    use {
        super::*,
        assert_matches::assert_matches,
        solana_program::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
        },
        solana_sdk::{signature::Signer, transaction::Transaction},
        solana_program_test::{processor, tokio, ProgramTest}
    };
    
    #[tokio::test]
    async fn test_vector_init_macro() {
        let program_id = Pubkey::new_unique();
        println!("vector init macro program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![0],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
    
    #[tokio::test]
    async fn test_vector_init_no_cap_push() {
        let program_id = Pubkey::new_unique();
        println!("vector init no cap push program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![1],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
    
    #[tokio::test]
    async fn test_vector_init_cap_push() {
        let program_id = Pubkey::new_unique();
        println!("vector init cap push program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![2],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    #[tokio::test]
    async fn test_for_loop_no_iter_no_ref_assign() {
        let program_id = Pubkey::new_unique();
        println!("for loop no iter no ref simple type assign program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![3],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    #[tokio::test]
    async fn test_for_iter_no_ref_assign() {
        let program_id = Pubkey::new_unique();
        println!("for loop iter no ref assign simple type program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![4],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    #[tokio::test]
    async fn test_for_no_iter_no_ref_complex_type_assign() {
        let program_id = Pubkey::new_unique();
        println!("for loop no iter no ref assign complex type program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![5],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    #[tokio::test]
    async fn test_for_iter_no_ref_complex_type_assign() {
        let program_id = Pubkey::new_unique();
        println!("for loop iter no ref assign complex type program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![6],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }


    #[tokio::test]
    async fn test_for_no_iter_ref_simple_type_assign() {
        let program_id = Pubkey::new_unique();
        println!("for loop no iter ref assign simple type program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![7],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    #[tokio::test]
    async fn test_for_iter_ref_simple_type_assign() {
        let program_id = Pubkey::new_unique();
        println!("for loop iter ref assign simple type program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![8],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    #[tokio::test]
    async fn test_pubkey_argument_pass_no_ref() {
        let program_id = Pubkey::new_unique();
        println!("pubkey argument pass no ref program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![9],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }

    #[tokio::test]
    async fn test_pubkey_argument_pass_ref() {
        let program_id = Pubkey::new_unique();
        println!("pubkey argument pass ref program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![10],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
    #[tokio::test]
    async fn test_pubkey_default_multicall() {
        let program_id = Pubkey::new_unique();
        println!("pubkey default multicall program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![11],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
    #[tokio::test]
    async fn test_pubkey_default_variable() {
        let program_id = Pubkey::new_unique();
        println!("pubkey default variable program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![12],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
    #[tokio::test]
    async fn test_print_string_format_args() {
        let program_id = Pubkey::new_unique();
        println!("pubkey default variable program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![13],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
    #[tokio::test]
    async fn test_print_string_format() {
        let program_id = Pubkey::new_unique();
        println!("pubkey default variable program id test {}", program_id);
        let pt = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        );
    
        let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    
        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![14],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
}