use std::io::{self, Write};

fn main() {
    main_loop();
}

fn main_loop () {
    loop {
        menu();

        //laskutoimituksen valinta:
        print!("Choose an option (1-5): ");
        io::stdout().flush().unwrap();
        let option = read_input();

        if option == 5 {
            println!("Lopetetaan ohjelma...");
            break;
        }

        if option < 1 || option > 5 {
            println!("\nTuntematon valinta!\n");
            continue;
        }

        print!("Enter first number: ");
        io::stdout().flush().unwrap();
        let input1 = read_input();
        /*if option == 4 && input1 == 0 {
            println!("\nNolla ei saa olla jaettavana!\n");
            continue;
        }*/

        print!("enter second number: ");
        io::stdout().flush().unwrap();
        let input2 = read_input();
        if option == 4 && input2 == 0 {
            println!("\nNollalla ei voi jakaa!\n");
            continue;
        }

        control_flow(option, input1, input2);
    }
}

fn menu() {
    println!("--- Calculator ---");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Quit");
    println!("-----------------");
}

//käyttäjän syötteen tallentaminen:
fn read_input() -> i32 {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read input");

        // Yritetään muuttaa syöte luvuksi. Jos se onnistuu, palautetaan luku (Ok).
        // Jos tulee virhe (esim. kirjaimia), pyydetään uusi syöte (Err) kaatumisen sijaan.
        match buffer.trim().parse() {
            Ok(result) => return result,
            Err(_) => {
                print!("Virheellinen syöte, syötä numero: ");
                io::stdout().flush().unwrap();
            }
        }
    }
}

//laskimen laskutoimitusten funktiot:
fn add(input1: i32, input2: i32) {
    let result = input1 + input2;
    println!("\nResult: {result}\n");
}

fn subtract(input1: i32, input2: i32) {
    let result = input1 - input2;
    println!("\nResult: {result}\n");
}

fn multiply(input1: i32, input2: i32) {
    let result = input1 * input2;
    println!("\nResult: {result}\n");
}

fn divide(input1: i32, input2: i32) {
    let result = input1 as f32 / input2 as f32;
    println!("\nResult: {result}\n");
}

//laskimen laskutoimitusten funktioiden käytön logiikka:
fn control_flow(option: i32, input1: i32, input2: i32) {
    if option == 1 {
        add(input1, input2);
    } else if option == 2 {
        subtract(input1, input2);
    }
    else if option == 3 {
        multiply(input1, input2);
    }
    else {
        divide(input1, input2);
    };
}