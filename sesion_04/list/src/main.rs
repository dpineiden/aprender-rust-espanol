// Importamos la librería fs para poder leer el archivo de la base de datos
use std::fs;

// Importamos la librería io para poder leer los archivos
use std::io;

// Importamos la librería unix::fs::PermissionsExt para poder ver los permisos del archivo
use std::os::unix::fs::PermissionsExt;

// Importamos la librería Chrono para manejar las fechas
use chrono::{DateTime, Local};

// Importamos la librería prettytable para manejar las tablas
use prettytable::{Table, row};

// Creamos la función main
// El tipo io::Result<()> indica que la función puede retornar un error
// relacionado con la entrada/salida y no retorna ningún valor en particular
fn main() -> io::Result<()> {
    // Creamos una variable llamada paths y obtenemos una lista de los archivos y carpetas en el directorio actual
    let paths = fs::read_dir(".")?;

    // Creamos una variable mutable llamada table y creamos una tabla
    let mut table = Table::new();

    // Añadimos una fila de encabezado con los títulos de las columnas
    table.add_row(row!["Nombre", "Permisos", "Fecha de Modificación", "Tamaño"]);

    // Creamos un ciclo for para iterar sobre cada uno de los archivos y carpetas
    for path in paths {

        // Obtenemos el path del archivo o carpeta actual
        let path = path?.path();

        // Obtenemos la metadata del archivo o carpeta actual
        let metadata = fs::metadata(&path)?;

        // Obtenemos los permisos del archivo o carpeta actual
        let permissions = metadata.permissions().mode();

        // Obtenemos la fecha de modificación del archivo o carpeta actual
        let modified = metadata.modified()?;

        // Obtenemos el tamaño del archivo o carpeta actual
        let file_size = metadata.len();

        // Creamos una fecha y hora con la fecha de modificación obtenida anteriormente
        let datetime = DateTime::<Local>::from(modified);

        // Formateamos la fecha y hora para que tenga el formato deseado
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        // Añadimos una nueva fila a la tabla con la información del archivo o carpeta actual
        table.add_row(row![path.display(), permissions, formatted_time, file_size]);
    }
    // Imprimimos la tabla en la consola
    table.printstd();

    // Retornamos Ok indicando que el programa ha finalizado correctamente
    Ok(())
}

