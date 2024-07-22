extern crate ndarray;
use ndarray::Array;

fn main() {

    println!("Hello, world!");

    let array1 = Array::from_shape_vec((2,2),vec![1.,2.,3.,4.]).unwrap();
    let array2 = Array::from_shape_vec((2,2),vec![4.,3.,2.,1.]).unwrap();

    println!("Array1:\n {:?}",array1);
    println!("Array2:\n {:?}",array2);


    let sum = &array1 + &array2;
    println!("Sum:\n {:?}",sum);


    let product = &array1 *(&array2);
    println!("Product:\n {:?}",product);

}
