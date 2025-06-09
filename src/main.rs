use std::{
    io::{self, Write},
    process::Command,
};

// The main drive
const DRIVE: &str = "/dev/vda";
const EFI_SIZE: u32 = 4096; // 4 GiB in Mib
const SWAP_SIZE: u32 = 32768; // 32 Gib in Mib

fn main() {
    println!("Welcome to plastic taste");
    // Here the user should be able to select a drive

    // partioning();
}

/// This function will format the partitions
fn formating() {
    let mut efi_part = DRIVE.to_owned();
    efi_part.push_str("1"); // /dev/vda1

    // For the EFI partition
    let status = Command::new("mkfs.fat")
        .args(["-F", "32", efi_part.as_str()])
        .status()
        .expect("mkfs.fat faild to format");
}

/// This function partitions the drives
/// This function works
fn partioning() {
    let efi_start = 1;
    let efi_end = efi_start + EFI_SIZE;

    let swap_start = efi_end;
    let swap_end = swap_start + SWAP_SIZE;

    let root_start = swap_end;

    println!("WARNING!!. Your about to WIPE ALL DATA on the selected drive: {DRIVE}");
    println!("Are you sure you want to continue?(y | n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => println!("Error!! getting input: {err}"),
    }

    let input = input.trim();
    match input {
        "yes" | "Yes" | "y" | "Y" => {
            // Very long args fix this later to make it look nicer?
            let status = Command::new("parted")
                .args([
                    DRIVE,
                    "--script",
                    "mklabel",
                    "gpt",
                    "mkpart",
                    "efi",
                    "fat32",
                    &format!("{efi_start}mib"),
                    &format!("{efi_end}mib"),
                    "name",
                    "1",
                    "boot",
                    "set",
                    "1",
                    "esp",
                    "on",
                    "mkpart",
                    "primary",
                    "linux-swap",
                    &format!("{swap_start}mib"),
                    &format!("{swap_end}mib"),
                    "name",
                    "2",
                    "swap",
                    "mkpart",
                    "primary",
                    "btrfs",
                    &format!("{root_start}mib"),
                    "100%",
                    "name",
                    "3",
                    "root",
                ])
                .status()
                .expect("Failed to run parted");

            if status.success() {
                println!("Partitioning completed successfully.");
            } else {
                println!("parted exited with error.");
            }
        }
        _ => {
            println!("Aborted. You must type 'y' or 'yes' to continue.");
        }
    }
}
