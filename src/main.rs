use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs; // fs es filesystem, ayudara a leer el archivo

// Estructura csv: TIPO | TAG | TEXTO | VIDA
const FILENAME: &str = "history.csv";

const FIRST_TAG: &str = "INICIO";

#[derive(Debug)] // ESto fue para poder imprimirlo
struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
}

// funcion generadora a DatoHistoria
impl DatoHistoria {
    /*
    Esto es muy interesante, impl es una implementacion a un TIPO DE DATO,
    en este caso se esta creando una implementacion para el tipo de dato DatoHistoria.

    Mientras que una funcion de una implementacion no contenga self, &self, o &mut self en su primer
    argumento, esa funcion se llamara de la siguiente forma: TipoDeDato::funcion().

    Si la funcion si contiene uno de esos argumentos en su primer argumento, esta podria ser llamada
    tambien con la sintaxis de llamar un metodo, como TipoDeDato.funcion()
    */
    fn new(row: StringRecord) ->  DatoHistoria{
        let vida: i32 = row.get(3).unwrap().trim().parse().unwrap_or(0);
        return DatoHistoria {
            tipo_dato: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida,
            opciones: vec![],
        };
    }
}

fn main() {

    let mut vida = 100;
    let mut tag_actual = FIRST_TAG;

    let mut last_record: String = "".to_string();

    // Declarando un array de DatoHistoria
    // let mut datos_historia: Vec<DatoHistoria> = vec![]; // Inicializando el vector vacio

    let mut datos_historia: HashMap<String, DatoHistoria> = HashMap::new();

    // leyendo el contenido del archivo y guardandolo en content como String
    let content = fs::read_to_string(FILENAME).unwrap();

    // Se usa el readerbuilder para traer no solo el string como tal sino la forma columnar que tienen los csv
    // devolviendo un resultado con varias filas
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        // println!("{:?}", result);

        let result = result.unwrap();

        // El unwrap_or lo que haria es que, en caso de haber error, devuelve el valor que le decimos
        // Se usa porque no en todas las opciones del csv se retorna una cantidad de vida
        
        // Llamando la implementacion que creamos para DatoHistoria
        let dato = DatoHistoria::new(result);
        if dato.tipo_dato == "SITUACION" {
            let record_tag = dato.tag.clone();
            datos_historia.insert(record_tag.clone(), dato);
            last_record = record_tag;
        } else if dato.tipo_dato == "OPCION" {
            if let Some(data) = datos_historia.get_mut(&last_record) {
                (*data).opciones.push(dato);
            }
        }
        // datos_historia.insert(dato.tag.clone(), dato); // clone() es para que no ocurra el error de partially moved
    }

    // Game loop
    loop {
        println!("Tienes {} de vida", vida);

        if let Some(data) = datos_historia.get(tag_actual) {
            println!("{}", data.texto);
            for (index, option) in data.opciones.iter().enumerate() {
                println!("[{}] {}", index, option.texto);
            }

            let mut seleccion = String::new();
            std::io::stdin().read_line(&mut seleccion).unwrap();
            let seleccion = seleccion.trim().parse().unwrap_or(99);

            if let Some(opcion_elegida) = &data.opciones.get(seleccion) {
                tag_actual = &opcion_elegida.tag;
            } else {
                println!("Comando no valido");
            }

            vida += data.vida;
            println!("");
        } else {
            break;
        }

        // Si la vida <= 0 terminar juego
        if vida <= 0 {
            println!("Has perdido!");
            break;
        }
    }

}
