use rand;
use rand::{Rng};
use std::io;

fn main() {

    let die01 = ["R","I","F","O","B","X"];
    let die02 = ["I","F","E","H","E","Y"];
    let die03 = ["D","E","N","O","W","S"];
    let die04 = ["U","T","O","K","N","D"];
    let die05 = ["H","M","S","R","A","O"];
    let die06 = ["L","U","P","E","T","S"];
    let die07 = ["A","C","I","T","O","A"];
    let die08 = ["Y","L","G","K","U","E"];
    let die09 = ["Qu","B","M","J","O","A"];
    let die10 = ["E","H","I","S","P","N"];
    let die11 = ["V","E","T","I","G","N"];
    let die12 = ["B","A","L","I","Y","T"];
    let die13 = ["E","Z","A","V","N","D"];
    let die14 = ["R","A","L","E","S","C"];
    let die15 = ["U","W","I","L","R","G"];
    let die16 = ["P","A","C","E","M","D"];
    
    let mut vec = vec![die01,die02,die03,die04,
        die05,die06,die07,die08,
        die09,die10,die11,die12,
        die13,die14,die15,die16];

    let mut fs = true;
    let mut input = String::new();

    println!("welcome to boggle letter generator");

    while fs
    {
        println!("enter 1 to roll");
        println!("enter 2 to exit");
        input.clear();
        io::stdin().read_line(&mut input).expect("wrong format");
        let answer = input.trim();
        if answer == "1"
        {
            let vec1 = gerenate(&mut vec);
            vec.push(die01);
            vec.push(die02);
            vec.push(die03);
            vec.push(die04);
            vec.push(die05);
            vec.push(die06);
            vec.push(die07);
            vec.push(die08);
            vec.push(die09);
            vec.push(die10);
            vec.push(die11);
            vec.push(die12);
            vec.push(die13);
            vec.push(die14);
            vec.push(die15);
            vec.push(die16);

            display(vec1);
        }
        else if answer == "2"
        {
            fs = false;
        }
        else
        {
            println!("that wasn't one of the options")
        }

    }
}

pub fn gerenate(vec :&mut Vec<[&str; 6]>) ->  Vec<String>
{
    let mut r = rand::thread_rng();
    let mut vec1 = vec!["".to_string()];    
    vec1.remove(0);
    for n in 0..16
    {
        let ran = r.gen_range(0, 16-n);
        let ran2 = r.gen_range(0,6);
        let bob = vec[ran][ran2];
        vec1.push(bob.to_string());
        vec.remove(ran);
    }
    return vec1;
}

pub fn display(vec1 :Vec<String>)
{
    println!("{} {} {} {}", vec1[0],vec1[1],vec1[2],vec1[3]);
    println!("{} {} {} {}", vec1[4],vec1[5],vec1[6],vec1[7]);
    println!("{} {} {} {}", vec1[8],vec1[9],vec1[10],vec1[11]);
    println!("{} {} {} {}", vec1[12],vec1[13],vec1[14],vec1[15]);
}











