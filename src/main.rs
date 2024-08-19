#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use core::num;
use std::fs;
use std::fs::File;
use std::io::Read;
use colored::Colorize;

pub struct mtrLexer
{
    inputBuffer :Vec<u8>,
    readOffset  :usize,
    carac       :u8,
} 

impl mtrLexer
{
    pub fn new(input:Vec<u8>) -> mtrLexer
    {
        let mut lexer = mtrLexer 
        {
            inputBuffer : input.clone(),
            readOffset  : 0,
            carac       : 0u8
        };            

        lexer.readChar();
        lexer
    }

    pub fn isFinished(&mut self) -> bool
    {
        if  self.carac == 0 {true}
        else                {false}  
    }

    fn skipWhiteSpace(&mut self)
    {
        while self.carac.is_ascii_whitespace()
        {
            self.readChar();
        }
    }

    fn readChar(&mut self)
    {
        if self.readOffset >= self.inputBuffer.len()
        {
            self.carac = 0;
        }
        else 
        {
            self.carac = self.inputBuffer[self.readOffset];
        }
        self.readOffset += 1;  
    }

    fn peekChar(&mut self) -> u8
    {
        if self.readOffset >= self.inputBuffer.len()
        {
            0
        }
        else 
        {
            self.inputBuffer[self.readOffset]
        }
    }

    pub fn display(&mut self)
    {
        //println!("buffer     is {:?}", self.inputBuffer);
        println!("buffer     is :");
        HexaDump(&self.inputBuffer);
        println!("carac      is {:?}|{:#04X}", self.carac, self.carac);
        println!("readOffset is {:?}", self.readOffset);
    }

    fn  isLetter(ch :u8) -> bool
    {
        ch.is_ascii_alphabetic()    
    }

    fn  isNumber(ch :u8) -> bool
    {
        ch.is_ascii_digit()
    }
    
    fn  readIdentifier(&mut self) -> String
    {
        let mut identifier = String::new();

        while mtrLexer::isLetter(self.carac)
        {
            identifier.push(self.carac as char);
            self.readChar();
        }

        identifier
    }

    fn  readNumber(&mut self) -> String
    {
        let mut number = String::new();

        while mtrLexer::isNumber(self.carac)
        {
            number.push(self.carac as char);
            self.readChar();
        }

        number
    }

    pub fn nextToken(&mut self)
    {
        self.skipWhiteSpace();
        
        let key:char = self.carac as char;
        
        let token = match self.carac 
        {
            b'='    =>  {
                            if self.peekChar()== b'='
                            {
                                self.readChar();
                                println!("Token is '=='");
                                "=="
                            }
                            else 
                            {
                                println!("Token is '='");
                                "="
                            }
                        },

            b';'    =>  {
                            self.readChar();
                            println!("Token is ';'");
                            ";"
                        },              

            b'('    =>  {
                            self.readChar();
                            println!("Token is '('");
                            "("
                        },              

            b')'    =>  {
                            self.readChar();
                            println!("Token is ')'");
                            ")"
                        },              

            b'{'    =>  {
                            self.readChar();
                            println!("Token is 'accolade ouvrante'");
                            "{"
                        },              

            b'}'    =>  {
                            self.readChar();
                            println!("Token is 'accolade fermante'");
                            "}"
                        },              

            
            _       =>  {
                            if mtrLexer::isLetter(self.carac) 
                            {
                                let identifier = self.readIdentifier(); 
                                println!("Token is an identifier {}", identifier);
                                "identifier"
                            }
                            else if mtrLexer::isNumber(self.carac)
                            {
                                let number = self.readNumber();
                                println!("Token is a number {}", number);
                                "number"
                            }
                            else 
                            {
                                println!("Token is unknown {}", self.carac as char);
                                self.readChar();
                                "unknown"
                            }
                        },              
        };
       
    }
}

fn HexaDump(toDisplay:&Vec<u8>)
{
    let mut index =0;
    let mut ascii: std::string::String = std::string::String::new();
    
    for element in toDisplay
    {
        print!("{:#04X} ", *element);
        let mut key:char = *element as char;
        if key.is_alphanumeric()==false {key='.';}
        ascii += &key.to_string();    
        //ascii += &element.to_owned().to_string();
        index +=1;
        if index%16==0  {print!("|{}|\n", ascii); ascii="".to_string();}
    }

    if index%16!=0
    {
        loop
        {
            index +=1;
            print!("     ");    
            ascii+=" ";
            if index%16==0   {break;}                    
        }
        print!("|{}|\n", ascii);
    }
}


fn main() 
{
    welcomeBanner();
    
    let pathProcFile = "C:\\GIT\\rustLexer\\maps\\admin.proc";
    //let contenuProcFile:Vec<u8> = fs::read(pathProcFile).expect("Can't read file");
    let contenuProcFile = fs::read_to_string(pathProcFile).expect("Can't read file");
    //HexaDump(&contenuProcFile);     // borrow it (by reference) to avoid movement...
    //println!("{}", contenuProcFile);
    let slices = contenuProcFile.split("\n");
    let mut index =0;
    for part in slices
    {
        println!("[{:04}] {}", index, part);
        index += 1;

        if index > 100  {break;}
    }
    
    return;
    //let path = "C:\\GIT\\essaiRust\\hello\\essai02\\fileToRead.txt";
    let pathMtrFile = "C:\\GIT\\rustLexer\\materials\\alphalabs.mtr";
    let contenuMtrFile:Vec<u8> = fs::read(pathMtrFile).expect("Can't read file");

    //let val = contenu.get(0);
    //let mut index = 0;

    //println!("contenu ={:?}", contenu);
    //print!("\n");
    //print!("\n");
    //HexaDump(&contenu);     // borrow it (by reference) to avoid movement...
    //print!("\n");
    //print!("\n");

    //let mut f               = File::open(&path).expect("no file found");
    //let metadata        = fs::metadata(&path).expect("unable to read metadata");
    //let mut buffer       = vec![0; metadata.len() as usize];
    //f.read(&mut buffer).expect("buffer overflow");

    /* //for element in buffer
    for (index, element) in buffer.into_iter().enumerate()
    {
        print!("{:#04X} ", element);
        if (index+1)%16==0  {print!("\n");}
    }
     */
    //HexaDump(&buffer);      // borrow it (by reference) to avoid movement...

    let mut ourMaterialLexer = mtrLexer::new(contenuMtrFile);
    //ourLexer.readChar();
    //println!("{}", ourLexer.peekChar() as char);
    ourMaterialLexer.display();
    while ourMaterialLexer.isFinished()==false
    {
        ourMaterialLexer.nextToken();
    }
    
}

fn thisExampleLexer(caInput:&Vec<u8>)
{
    let mut index:usize =0;

    loop
    {
        if caInput.get(index).is_none() {break;}
        let carac = caInput.get(index).expect("Out of range");
        let key:char = *carac as char;
        
        match *carac 
        {
            0x30u8..=0x39u8     => println!("Number    {}", key), 
            b'A'..=b'Z'         => println!("UpperCase {}", key),                  
            b'a'..=b'z'         => println!("LowerCase {}", key),                  
            0x0D                => println!("void"),
            0x0A                => println!("CRLF"),
            _                   => println!("default   {}", key),                    
        }

        index += 1;
    }
}

fn welcomeBanner()
{
    print!("\n");
    print!("\n");
    
    println!(
        "{}, {}, {}, {}, {}, {}, and some normal text.",
        "Bold".bold(),
        "Red".red(),
        "Yellow".yellow(),
        "Green Strikethrough".green().strikethrough(),
        "Blue Underline".blue().underline(),
        "Purple Italics".purple().italic()
    );

    println!("{}", "this is blue".blue());
    println!("{}", "this is red".red());
    println!("{}", "this is red on blue".red().on_blue());
    println!("{}", "this is also red on blue".on_blue().red());
    println!("{}", "you can use truecolor values too!".truecolor(0, 255, 136));
    println!("{}", "background truecolor also works :)".on_truecolor(135, 28, 167));
    println!("{}", "you can also make bold comments".bold());
    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    println!("{}", "or change advice. This is red".yellow().blue().red());
    println!("{}", "or clear things up. This is default color and style".red().bold().clear());
    println!("{}", "purple and magenta are the same".purple().magenta());
    println!("{}", "bright colors are also allowed".bright_blue().on_bright_white());
    println!("{}", "you can specify color by string".color("blue").on_color("red"));
    println!("{}", "and so are normal and clear".normal().clear());
    //String::from("this also works!").green().bold();
    //format!("{:30}", "format works as expected. This will be padded".blue());
    //format!("{:.3}", "and this will be green but truncated to 3 chars".green());

    //let mut uiFrancky : u32 = 0xdeadbeef;
    let uiFrancky : u32 = 0xdeadbeef;
    println!("Hello, world! {uiFrancky:#10X}");
    println!("              {uiFrancky:#b}");
    print!("\n");
    print!("\n");

}