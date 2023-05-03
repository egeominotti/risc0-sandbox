// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{PW_CHECK_ELF, PW_CHECK_ID};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::sha::Digest;
use risc0_zkvm::Prover;

fn main() {
    // Make the prover.
    let mut prover =
        Prover::new(PW_CHECK_ELF).expect("Prover should be constructed from valid ELF binary");

    // TODO: Implement communication with the guest here
    let pw = String::from("verystrongpassword!");
    prover.add_input_u32_slice(&to_vec(&pw).expect("should be serializable"));

    // Run prover & generate receipt
    let receipt = prover.run().expect(
        "Code should be provable unless it had an error or exceeded the maximum cycle limit",
    );

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(&PW_CHECK_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    let digest: Digest = from_slice(&receipt.get_journal_bytes()).unwrap();
    println!(
        "We proved this hash {} came from a pw with special characters",
        digest
    );
}
