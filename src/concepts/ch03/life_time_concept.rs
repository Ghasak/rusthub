pub fn life_time_concept_fn() {

    let some_int_var : i32 = 10;
    let reference_int_val = &some_int_var;
    let result_ref: &i32;
    {
        result_ref = get_int_ref(reference_int_val );
    }
    println!("{result_ref}");

}


fn get_int_ref<'a>(param_1: &'a i32)-> &'a i32{
    param_1
}
