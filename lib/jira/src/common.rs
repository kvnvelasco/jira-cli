use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedResponse<T> {
    pub values: Vec<T>,
}

pub trait Paginated {
    fn get_distance_from_top(&self) -> usize;
    fn get_number_of_pages(&self) -> usize;
}

// impl<T> Paginated for PaginatedResponse<T> {
//   fn get_distance_from_top(&self) -> usize {
//     let total = self.startAt + self.values.len();
//     if self.total > total {
//       self.total - total
//     } else {
//       0
//     }
//   }

//   fn get_number_of_pages(&self) -> usize {
//     match self.get_distance_from_top() {
//       0 => 0,
//       _ => {
//         let distance = self.get_distance_from_top();
//         let page_size = self.maxResults;
//         if distance % page_size == 0 {
//           (distance / page_size)
//         } else {
//           (distance / page_size) + 1
//         }
//       }
//     }
//   }
// }
