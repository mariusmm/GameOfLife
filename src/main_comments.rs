// // #[derive(Clone)] permite qe las celdas se clonen
// // #[derive(Clone)] se usa para derivar automáticamente la implementación del trait Clone para una estructura o enum
// // Básicamente crea la copia de un valor, sin tendrías que hacer algo como:

// // impl Clone for Cell {
// //     fn clone(&self) -> Cell {
// //         Cell { alive: self.alive }
// //     }
// // }

// #[derive(Clone)]
// struct Cell {
//     alive: bool,
// }
    
// // Contiene la estructura del tablero de juego.
// // Contiene una lista de celdas y las dimensiones del tablero

// #[derive(Clone)]
// struct Board {
//     cells: Vec<Cell>,
//     width: usize,
//     height: usize,
// }

// // El método new inicializa un nuevo tablero con todas las celdas muertas

// impl Board {
//     fn new(width: usize, height: usize) -> Board {
//         Board {
//             // vec! es una macro que se utiliza para crear un Vec<T>
//             cells: vec![Cell { alive: false }; width * height],
//             width,
//             height,
//         }
//     }

//     // El método get devuelve si una celda está viva o no en una posición
//     fn get(&self, x: usize, y: usize) -> bool {
//         // Calcula el índice unidimensional para la celda en la posición (x, y)
//         let index = y * self.width + x;
//         // Accede a la celda en ese índice y devuelve su estado (alive o dead)
//         self.cells[index].alive
//     }

//     // El método set cambia el estado de una celda en una posición específica
//     // &mut self es una referencia mutable, lo que significa que puede modificar el estado interno de la estructura Board
//     fn set(&mut self, x: usize, y: usize, alive: bool) {
//         self.cells[y * self.width + x].alive = alive;
//     }

//     // Para optimizar el código, podemos ajustar las funciones apply_rules y count_neighbors para que trabajen directamente con usize, eliminando la necesidad de convertir los índices en cada iteración del bucle.
    
//     // // El método count_neighbours cuenta cuántos vecinos vivos tiene una celda en una posición específica
//     // fn count_neighbors(&self, x: i32, y: i32) -> usize {
//     //     let mut count = 0; // cuenta las celdas vivas o muertas

//     //     for x_idx in [-1, 0, 1] { // estos bucles anidados reprensentan el desplazamiento en las coordenadas x e y para explorar las celdas vecinas de alrededor
//     //         for y_idx in [-1, 0, 1] {
//     //             if x + x_idx < 0 // || son el operador OR, si alguna es verdadera continue
//     //                 || x + x_idx >= self.width as i32
//     //                 || y + y_idx < 0
//     //                 || y + y_idx >= self.height as i32
//     //                 || x_idx == 0 && y_idx == 0

//     //                 // Estas condiciones miran:
//     //                 // Si la celda vecina está fuera del límite del tablero
//     //                 // Si (x_idx, y_idx) es (0,0) lo que significa la celda actual (no es na vecina)
//     //             {
//     //                 continue; // si se cumple seguimos
//     //             }
//     //             if self.get((x + x_idx) as usize, (y + y_idx) as usize) {
//     //                 count += 1;
//     //             }
//     //         }
//     //     }
//     //     count
//     // }

//     fn count_neighbors(&self, x: usize, y: usize) -> usize {
//         let mut count = 0;
    
//         for x_idx in [-1, 0, 1].iter().cloned() {
//             for y_idx in [-1, 0, 1].iter().cloned() {
                
//                 let nx = x as isize + x_idx;
//                 let ny = y as isize + y_idx;
    
//                 if nx < 0 
//                 || nx >= self.width as isize 
//                 || ny < 0 
//                 || ny >= self.height as isize 
//                 || (x_idx == 0 && y_idx == 0) 
                
//                 {
//                     continue;
//                 }
    
//                 if self.get(nx as usize, ny as usize) {
//                     count += 1;
//                 }
//             }
//         }
    
//         count
//     }

    
//     // // Esta función aplica las reglas del Juego de la Vida para determinar si una celda vivirá o morirá en la próxima generación
//     // // https://es.wikipedia.org/wiki/Juego_de_la_vida
//     // fn apply_rules(&self, x: i32, y: i32) -> bool {

//     //     let num_neigh = self.count_neighbors(x, y);

//     //     if self.get(x as usize, y as usize) {
//     //         match num_neigh {
//     //             0 => false, // la celda muere
//     //             1 => false, // la celda muere
//     //             2 => true, // la celda sigue viva
//     //             3 => true, // la celda sigue viva
//     //             _ => false, // más de 3 vecinos, la celda muerte
//     //         }
//     //     } else {
//     //         num_neigh == 3
//     //     }
//     // }

//     fn apply_rules(&self, x: usize, y: usize) -> bool {
//         let num_neigh = self.count_neighbors(x, y);
    
//         if self.get(x, y) {
//             match num_neigh {
//                 0 => false,
//                 1 => false,
//                 2 => true,
//                 3 => true,
//                 _ => false,
//             }
//         } else {
//             num_neigh == 3
//         }
//     }
    

//     // fn evolve(&self) -> Board {
//     //     // Clonar el tablero actual para crear una nueva generación sin modificar el estado actual
//     //     let mut new_board = self.clone();
    
//     //     // En términos de optimización en Rust, el uso de conversiones de tipos entre usize e i32 puede ser ineficiente si se realiza repetidamente, especialmente en bucles anidados que se ejecutan muchas veces. Es mejor evitar conversiones de tipos innecesarias cuando sea posible.

//     //     // Iterar sobre todas las celdas del tablero
//     //     for y in 0..self.height as i32 {
//     //         for x in 0..self.width as i32 {
//     //             // Aplicar las reglas del juego a la celda actual para determinar su estado en la siguiente generación
//     //             let alive = self.apply_rules(x, y);
//     //             // Actualizar el nuevo tablero con el estado calculado
//     //             new_board.set(x as usize, y as usize, alive);
//     //         }
//     //     }
    
//     //     // Devolver el nuevo tablero después de aplicar las reglas a todas las celdas
//     //     new_board
//     // }
    
//     fn evolve(&self) -> Board {
//         let mut new_board = self.clone();
    
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 let alive = self.apply_rules(x, y);
//                 new_board.set(x, y, alive);
//             }
//         }
    
//         new_board
//     }

//     // Optimizaciones aplicadas de conversión de tipos

//     // Eliminación de Conversiones de Tipo: Al ajustar las funciones count_neighbors y apply_rules para que trabajen con usize, eliminamos la necesidad de realizar conversiones repetidas entre usize e i32.
//     // Iteración Eficiente: Ahora iteramos directamente sobre las coordenadas del tablero utilizando usize, lo que mejora la eficiencia.
//     // Código más Limpio y Eficiente: Estas optimizaciones no solo hacen que el código sea más eficiente, sino también más limpio y fácil de entender.

//     fn initialize_glider(&mut self) {
//         self.set(1, 0, true);
//         self.set(2, 1, true);
//         self.set(0, 2, true);
//         self.set(1, 2, true);
//         self.set(2, 2, true);
//     }

//     fn print(&self) {
//         println!("**************************************************************************");
//         for y_idx in 0..self.height {
//             for x_idx in 0..self.width {
//                 if self.get(x_idx, y_idx) {
//                     // Celda viva
//                     print!("█");
//                 } else {
//                     // Celda muerta
//                     print!("░");
//                 }
//             }
//             println!();
//         }
//     }
// }

// fn main() {
//     let mut my_board = Board::new(50, 50);

//     // Inicializar el tablero con un patrón Glider
//     my_board.initialize_glider();

//     // Imprimir el estado inicial
//     my_board.print();

//     // Evolucionar el tablero a través de varias generaciones
//     for _ in 0..100 {
//         my_board = my_board.evolve();
//         my_board.print();
//     }
// }
