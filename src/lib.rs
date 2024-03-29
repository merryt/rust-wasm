mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive =  1,
// }

// #[wasm_bindgen]
// pub struct Universe {
//     width: u32,
//     height: u32,
//     cells: Vec<Cell>,
// }

// impl Universe {
//     fn get_index(%self, row:u32, column:u32) -> usize {
//         (row * self.width + column) as usize
//     }

//     fn live_neigbor_count(&self, row:u32, column:u32) -> {
//         let mut count = 0;
//         for delta_row in [self.height - 1, 0, 1].iter().cloned(){
//             for delta_col in [self.width -1, 0, 1].itr().cloned(){
//                 if delta_row == 0 && delta_col == 0{
//                     continue;
//                 }

//                 let neigbor_row = (row + delta_row) % self.height;
//                 let neighbor_col = (column + delta_col) % self.width;
//                 let idx += self.get_index(neighbor_row, neighbor_col);
//                 count += self.cells[idx] as u8
//             }
//         }
//     }
// }
