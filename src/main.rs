use std::{
    fs,
    env,
    collections::HashMap,
    process::exit,
};
//el hecho de usar un hashmap hace que las variables no esten necesariamente en el orden en que fueron declaradas en el stack
const direc_base_var:usize = 16;
const simbolos_precargados: [(&'static str,usize);22] = [
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
    if argumentos.len() != 1{
        println!("Expected one command-line argument with the name of the file to compile");
        exit(-1);
    } 
    let archivo = match fs::read_to_string(argumentos.pop().unwrap()){
        Ok(valor)=>valor,
        Err(_)=>{println!("It was not possible to access the file");exit(-2)},
    };
    let mut tabla: HashMap<String,VarType> = simbolos_precargados.iter().map(|x| (x.0.to_string(),VarType::Variable(x.1))).collect(); 
    let solo_instrucciones = procesar_simbolos(archivo,&mut tabla);
    println!("{}",solo_instrucciones);


}
#[derive(PartialEq)]
enum VarType{
    Undefined,
    Variable(usize),
    Loop(usize),
}
impl VarType{
    #[inline]
    fn unwrap(&self)->usize{
        match self{
            VarType::Loop(valor)=>*valor,
            VarType::Variable(valor)=>*valor,
            VarType::Undefined=>panic!("Unwrap on VarType::Undefined"),
        }
    }
}

fn procesar_simbolos(archivo:String,tabla:&mut HashMap<String,VarType>)->String{
    let mut parseado = String::new();
    let mut n_instruccion = 0_usize;
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
            n_instruccion += 1;
        }
    }
    let mut i = direc_base_var;
    for coso in tabla.values_mut(){
        if *coso == VarType::Undefined{
            *coso = VarType::Variable(i);
            i += 1;
        }
    }
    parseado
}