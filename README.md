# boggle_rust
a boggle word board

this application was inspired by one of my favorite word games, boggle.  
one of the most fun things about the game is shaking up the letter dice  
and then letting them settle. but the shaking causes as sorts of noise,  
so I made a program for those with more sensitive ears that allows you to generate the letters for boggle  
  
## setting up the enviorment  
  
this program was written in rust, this is the first program I ever wrote in rust and there definitly  
are some different syntax trope that i had to get used to, but first things first,   
  
to set up the enviorment i had to download rust from its website (listed below)  

after it downloads a .exe file there will be a small console that opens up,  
and it will prompt you to either install rust with the default settings,(which is what I did) or to customize them.  
  
after download you create your first program by opening your comand prompt and  
navigate to the file you want your program in. when you get there you type  
  
cargo new "name of the program you want"  
  
and then rust will create a new folder system for you with a source file and a couple other system required files.  
  
I used vs code for my ide, and I just had to download the rust extention to use this program.  
  
to test your program you open up your terminal and after saving your file you type  

cargo run  
  
and it will build compile and run your program  
  
## how i built my program  

this program was built to demonstrate the use of

* arrays
* variables
* loops
* functions
* vectors
* passing by refrence or borrowing
* rand

this program has a main function along with two other functions.  
one to generate a boggle board   
and another one to display it  

this program is the shortest of the programs i have written in rust  
but it was the most complicated to solve becuase of the  
attributes of the vectors and arrays

I also used a random number generator, and so i had to add
rand = "0.7"
under the [dependencies]
in the cargo.toml file

## my program in action  

when you first start the program you will be given two options

![instructions example](https://github.com/bshort95/boggle_rust/blob/main/bogCapture1.JPG?raw=true) 

you can choose to roll

![instructions example](https://github.com/bshort95/boggle_rust/blob/main/bogCapture2.JPG?raw=true)

or quit

but you can roll as many times as you like. and the roll is not random letters but they work as the dice do in the original wordgame 

## useful websites
[Git](https://git-scm.com/)  
[visual studios code](https://code.visualstudio.com/)  
[rust website](https://www.rust-lang.org/)  
[youtube](https://www.youtube.com/)  
[boogle info](https://en.wikipedia.org/wiki/Boggle)

