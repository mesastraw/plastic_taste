use std::{
    io::{self, Write},
    process::{Command, exit},
};

// The main drive
// This wont be a const in the future
// User will be a able to choose a drive later on
const DRIVE: &str = "/dev/vda";
const EFI_SIZE: u32 = 4096; // 4 GiB in Mib
const SWAP_SIZE: u32 = 32768; // 32 Gib in Mib

fn main() {
    println!("Welcome to plastic taste");
    // Here the user should be able to select a drive

    partioning(DRIVE);
    formating(DRIVE);
    mounting(DRIVE);
}

/// This function will deal with mounting all the drives
fn mounting(drive: &str) {
    // Mounting the root partition
    let root_part = [drive, "3"].concat();
    let status = Command::new("mount")
        .args([root_part, "/mnt".to_owned()])
        .status()
        .expect("Failed to start mount");

    // This repeats a lot should probably make it a function
    // Or change it to something else
    if status.success() {
        println!("Success mounting root")
    } else {
        println!("Failure mounting root");
        exit(-1)
    }

    let efi_part = [drive, "1"].concat();
    let status = Command::new("mount")
        .args(["--mkdir", efi_part.as_str(), "/mnt/boot"])
        .status()
        .expect("Failed to start mount");

    if status.success() {
        println!("Success mounting efi")
    } else {
        println!("Failure mounting efi");
        exit(-1)
    }

    let swap_part = [drive, "2"].concat();
    let status = Command::new("swapon")
        .arg(swap_part)
        .status()
        .expect("Failed to start swapon");

    if status.success() {
        println!("Success enabling swap")
    } else {
        println!("Failure enabling swap");
        exit(-1)
    }
}

// Works
// Handle errors better in the future
/// This function will format the partitions
fn formating(drive: &str) {
    // Formatting EFI partition
    let efi_part = [drive, "1"].concat();
    let status = Command::new("mkfs.fat")
        .args(["-F", "32", efi_part.as_str()])
        .status()
        .expect("error starting mkfs.fat");

    if status.success() {
        println!("Formatting efi partition suceeded")
    } else {
        println!("Formmatting efi failed");
        exit(-1)
    }

    // Formatting swap partition
    let swap_part = [drive, "2"].concat();
    let status = Command::new("mkswap")
        .arg(swap_part)
        .status()
        .expect("error running mkswap");

    if status.success() {
        println!("Formatting swap partition suceeded")
    } else {
        println!("Formmatting efi failed");
        exit(-1)
    }

    // Formatting root partition
    let root_part = [drive, "3"].concat();
    let status = Command::new("mkfs.btrfs")
        .args(["-f", root_part.as_str()])
        .status()
        .expect("Error running mkfs.btrfs");

    if status.success() {
        println!("Formatting root partition suceeded")
    } else {
        println!("Formmatting root failed");
        exit(-1)
    }
}

// Works
// Handle errors better in the future
/// This function partitions the drives
fn partioning(drive: &str) {
    let efi_start = 1;
    let efi_end = efi_start + EFI_SIZE;

    let swap_start = efi_end;
    let swap_end = swap_start + SWAP_SIZE;

    let root_start = swap_end;

    println!("WARNING!!. Your about to WIPE ALL DATA on the selected drive: {drive}");
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
                    drive,
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
            exit(0)
        }
    }
}
