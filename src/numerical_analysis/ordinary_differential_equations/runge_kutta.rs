pub fn basics_of_ndarray() {
    use ndarray::parallel::prelude::*;
    use ndarray::prelude::*;
    use rayon::prelude::*;

    let y: Array1<f64> = array![1.0, 2.0, 3.0, 4.0, 21.0, 32.0];
    //println!("{:#?}",f(10.0, &y));
    // ndarray accepting only references once being defined.
    let a: Array1<f64> = array![1.0, 2.0, 3.0, 4.0];
    let b: Array1<f64> = array![10.0, 20.0, 30.0, 40.0];
    let c: Array1<f64> = &a + &b;
    let d = &a.dot(&b);
    // println!("{:#?}", a);
    // println!("{:#?}", b);
    // println!("{:#?}", c);
    // println!("{:#?}", d);

    //ndarray Quick - Start
    // ## ndarray meaning
    let arr1: Array1<f64> = array![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let dt: f64 = 20.0;
    let result_1 = dt * &arr1;
    // println!("{:#?}", result_1);
    //let result_2 = f64::powf(result_1, 2.0);
    // let output: Vec<f64> = Vec::with_capacity(arr1.len())
    // for item in output.iter(){
    //     println!("{:#?}", output);
    // }
    // let output: Array1<f64> =array![1.0, 2.0, 3.0];
    // output.iter().map(|| -> x);
    //let output = &y.mapv_into(|v| f64::powf(v, 2.0));
    //let output_parallel = y.par_mapv_inplace(|v| f64::powf(v, 2.0));

    // println!("Value to be printed .. {:#?}", c);
    let a = Array::linspace(0., 63., 64).into_shape((4, 16)).unwrap();
    let c = a.clone().mapv_into(|v| f64::powf(v, 2.0));

    println!("{:#?}", a);
    println!("\n ******",);
    println!("{:#?}", c);
}

pub fn welcome() {
    println!("Hello form Rung-kutta");

    // This cargo is used as a replacement for numpy in pyhton
    // Using the API from here: https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html
    use ndarray::prelude::*;

    const T_0: f64 = 0.0;
    const Y_1: f64 = 0.0;
    const T_FINAL: f64 = 5.;
    const DT: f64 = 0.5;
    const DT_EXACT: f64 = 0.01;

    // fn f(t: f64, y: &Array1<f64>) -> Vec<f64> {
    //     let mut y_result: Vec<f64> = vec![];
    //     for item in y {
    //         //let angle : f64 = f64::to_degrees(*item);
    //         //let a = f64::powf(2.0, f64::sin(t)) * *item;
    //         //let a = f64::powf(f64::sin(t),2.0);
    //         let a = f64::sin(t).powf(2.0) * *item;
    //         y_result.push(a);
    //     }
    // }
}
