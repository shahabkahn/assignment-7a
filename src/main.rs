mod furniture {
     
    pub mod sofa {
         
       pub fn sofa_make () {
             println!("Made in Chiniot");
         }
    }
}

use crate::furniture::sofa;

fn main() {
   sofa::sofa_make();
}
