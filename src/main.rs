use std::{
    fs,
    env,
    collections::HashMap,
    process::exit,
    path::Path,
};
//el hecho de usar un hashmap hace que las variables no esten necesariamente en el orden en que fueron declaradas en el stack
const DIREC_BASE_VAR:i16 = 16;
const DESTINATION:[(&'static str,&'static str);8] = [
    ("None","000"),
    ("M","001"),
    ("D","010"),
    ("MD","011"),
    ("A","100"),
    ("AM","101"),
    ("AD","110"),
    ("AMD","111"),
];
const JUMP:[(&str,&str);8] = [
    ("None","000"),
    ("JGT","001"),
    ("JEQ","010"),
    ("JGE","011"),
    ("JLT","100"),
    ("JNE","101"),
    ("JLE","110"),
    ("JMP","111")
];
const OPERATIONS:[(&str,&str);29] = [
    ("None",""),
    ("0","0101010"),
    ("1","0111111"),
    ("-1","0111010"),
    ("D","0001100"),
    ("A","0110000"),
    ("M","1110000"),
    ("!D","0001101"),
    ("!A","0110001"),
    ("!M","1110001"),
    ("-D","0001111"),
    ("-A","0110011"),
    ("-M","1110011"),
    ("D+1","0011111"),
    ("A+1","0110111"),
    ("M+1","1110111"),
    ("D-1","0001110"),
    ("A-1","0110010"),
    ("M-1","1110010"),
    ("D+A","0000010"),
    ("D+M","1000010"),
    ("D-A","0010011"),
    ("D-M","1010011"),
    ("A-D","0000111"),
    ("M-D","1000111"),
    ("D&A","0000000"),
    ("D&M","1000000"),
    ("D|A","0010101"),
    ("D|M","1010101")
];
const SIMBOLOS_PRECARGADOS: [(&'static str,i16);22] = [
        ("SP",0),
        ("LCL",1),
        ("THIS",3),
        ("THAT",4),
        ("SCREEN",16384),
        ("KBD",24576),
        ("R0",0),
        ("R1",1),
        ("R2",2),
        ("R3",3),
        ("R4",4),
        ("R5",5),
        ("R6",6),
        ("R7",7),
        ("R8",8),
        ("R9",9),
        ("R10",10),
        ("R11",11),
        ("R12",12),
        ("R13",13),
        ("R14",14),
        ("R15",15)
        ];

fn main() {
    let mut argumentos = env::args().collect::<Vec<String>>();
    if argumentos.len() != 2{
        println!("Expected one command-line argument with the name of the file to compile");
        exit(-1);
    } 
    let argumentos = argumentos.pop().unwrap();
    let file = Path::new(&argumentos);
    let archivo = match fs::read_to_string(file){
        Ok(valor)=>valor,
        Err(_)=>{println!("It was not possible to access the file");exit(-2)},
    };
    let tabla: HashMap<String,VarType> = SIMBOLOS_PRECARGADOS.iter().map(|x| (x.0.to_string(),VarType::Loop(x.1))).collect(); 
    let mut compilado = String::new();
    for instruccion in procesar_simbolos(archivo,tabla){
        match instruccion.compile(){
            Ok(valor)=>{compilado.push_str(&valor);compilado.push('\n')},
            Err(_)=>{println!("Compilation error");exit(-10)},
        }
    }
    if let Err(_) = fs::write(file.with_extension("hack"), compilado){
        println!("Error when writting to the compiled file");
        exit(-3);
    }
}
#[derive(PartialEq,Copy,Clone)]
enum VarType{
    Undefined,
    Loop(i16),
}
impl VarType{
    #[inline]
    fn unwrap(&self)->i16{
        match self{
            VarType::Loop(valor)=>*valor,
            VarType::Undefined=>panic!("Unwrap on VarType::Undefined"),
        }
    }
}
#[derive(Debug)]
enum Instruction{
    A(i16),
    C(String),
}
impl Instruction{
    #[inline]
    fn compile(self)->Result<String,()>{
        match self{
            Instruction::A(valor)=>Ok(format!("{:016b}",valor)),
            Instruction::C(valor)=>{
                let mut cadena = String::from("111");
                let igual = valor.contains('=');
                let jmp = valor.contains(';');
                let valor = valor.split(|x| x=='=' || x==';').map(|x| x.trim().replace(" ","")).collect::<Vec<String>>();
                if igual{
                    let mut compilation_error = true;
                    for op in &OPERATIONS{
                        if op.0 == valor[1]{
                            cadena.push_str(op.1);
                            compilation_error = false;
                            break;
                        } 
                    }
                    if compilation_error{
                        return Err(());
                    }
                    compilation_error = true;
                    for dest in &DESTINATION{
                        if dest.0 == valor[0]{
                            cadena.push_str(dest.1);
                            compilation_error = false;
                            break;
                        }
                    }
                    if compilation_error{
                        return Err(());
                    }
                    if jmp{
                        compilation_error = true;
                        for jm in &JUMP{
                            if jm.0 == valor[2]{
                                cadena.push_str(jm.1);
                                compilation_error = false;
                                break;
                            }
                        }
                        if compilation_error{
                            return Err(());
                        }
                    }else{
                        cadena.push_str("000");
                    }
                }else if jmp{
                    let mut compilation_error = true;
                    for op in &OPERATIONS{
                        if op.0 == valor[0]{
                            cadena.push_str(op.1);
                            compilation_error = false;
                            break;
                        } 
                    }
                    if compilation_error{
                        return Err(());
                    }
                    cadena.push_str("000");
                    compilation_error = true;
                    for jm in &JUMP{
                        if jm.0 == valor[1]{
                            cadena.push_str(jm.1);
                            compilation_error = false;
                            break;
                        }
                    }
                    if compilation_error{
                        return Err(());
                    }

                }else{
                    return Err(());
                }
                Ok(cadena)
            }
        }
    }
}
#[inline]
fn procesar_simbolos(archivo:String,mut tabla:HashMap<String,VarType>)->Vec<Instruction>{
    let mut parseado = String::new();
    let mut n_instruccion = 0_i16;
    'outer: for line in archivo.lines(){
        let line = line.trim();
        if line.is_empty() || line.starts_with("//"){ //No se que tan eficiente es esto, seguro hay mejores opciones
            continue 'outer;
        }else if line.starts_with("@"){
            let mut line = line;
            if let Some(index) = line.find("//") {
                line = &line[..index].trim();
            }
            let mut flag = false;
            for caracter in line.chars().skip(1){
                if !caracter.is_digit(10){
                    flag = true;
                    break;
                }
            }
            if flag{
                let clave = &line[1..];
                if let None = tabla.get(clave) {
                    tabla.insert(clave.to_string(),VarType::Undefined);
                }
            }
            parseado.push_str(line);
            parseado.push('\n');
            n_instruccion += 1;
        }else if line.starts_with("("){
            let mut line = line;
            if let Some(index) = line.find("//") {
                line = &line[..index].trim();
            }
            line = &line.trim_matches(|x| x=='(' || x==')').trim();
            tabla.insert(line.to_string(),VarType::Loop(n_instruccion));  
        }else{
            let mut line = line;
            if let Some(index) = line.find("//") {
                line = &line[..index];
            }
            parseado.push_str(line);
            parseado.push('\n');
            n_instruccion += 1;
        }
    }
    let mut i = DIREC_BASE_VAR;
    let mut devolver: Vec<Instruction> = Vec::new();
    for line in parseado.lines(){
        if let Some(valor) = line.strip_prefix('@'){
            match valor.parse::<i16>(){
                Ok(valor)=>devolver.push(Instruction::A(valor)),
                Err(_)=>{
                    let valor_a = *tabla.get(valor).unwrap();
                    if valor_a == VarType::Undefined{
                        devolver.push(Instruction::A(i));
                        i +=1;
                    }else{
                        devolver.push(Instruction::A(valor_a.unwrap()))
                    }
                },
            }
        }else{
            devolver.push(Instruction::C(line.trim().to_string()));
        }
    }
    devolver
}