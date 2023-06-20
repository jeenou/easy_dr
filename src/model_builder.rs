
use jlrs::prelude::*;
use std::path::PathBuf;
use jlrs::memory::target::frame::GcFrame;
use jlrs::error::JlrsError;
use jlrs::data::managed::value::ValueResult;

// Calls a Julia function with the specified module name, function name, and one data argument.
// Returns the result of the function as an i64 integer, or an error if the call fails.
pub fn _call_julia_function1_old<'scope, 'a>(mut frame: GcFrame<'scope>, module: &str, function: &str, data1: Value<'scope, 'a>) -> Result<jlrs::prelude::Value<'scope, 'a>, std::boxed::Box<JlrsError>>
{
    unsafe {
        Module::main(&frame)
            .submodule(&frame, module)?
            .as_managed()
            .function(&frame, function)?
            .as_managed()
            //geneerinen call on olemassa myös
            .call1(&mut frame, data1)
            .into_jlrs_result()
    }
}

pub fn _call_julia_function1<'target, 'data, T: Target<'target>>(
    target: T, 
    module: &str, 
    function: &str, 
    data1: Value<'_, 'data>
) -> JlrsResult<ValueResult<'target, 'data, T>> {
    unsafe {
        let res = Module::main(&target)
            .submodule(&target, module)?
            .as_managed()
            .function(&target, function)?
            .as_managed()
            //geneerinen call on olemassa myös
            .call1(target, data1);

        Ok(res)
    }
}

pub fn _call_julia_function3<'target, 'data, T: Target<'target>>(
    target: T, 
    module: &str, 
    function: &str, 
    data_1: Value<'_, 'data>,
    data_2: Value<'_, 'data>,
    data_3: Value<'_, 'data>
) -> JlrsResult<ValueResult<'target, 'data, T>> {
    unsafe {
        let res = Module::main(&target)
            .submodule(&target, module)?
            .as_managed()
            .function(&target, function)?
            .as_managed()
            //geneerinen call on olemassa myös
            .call3(target, data_1, data_2, data_3);

        Ok(res)
    }
}


pub fn _call_julia_function2<'scope, 'a>(mut frame: GcFrame<'scope>, module: &str, function: &str, data1: Value<'scope, 'a>, data2: Value<'scope, 'a>) -> Result<jlrs::prelude::Value<'scope, 'a>, std::boxed::Box<JlrsError>>
{
    unsafe {
        Module::main(&frame)
            .submodule(&frame, module)?
            .as_managed()
            .function(&frame, function)?
            .as_managed()
            //geneerinen call on olemassa myös
            .call2(&mut frame, data1, data2)
            .into_jlrs_result()
    }
}

// Calls a Julia function with the specified module name, function name, and data argument.
// Returns the result of the function as an i64 integer, or an error if the call fails.
pub fn _call_julia_function3_old<'scope, 'a>(mut frame: GcFrame<'scope>, module: &str, function: &str, data1: Value<'scope, 'a>, data2: Value<'scope, 'a>, data3: Value<'scope, 'a>) -> Result<jlrs::prelude::Value<'scope, 'a>, std::boxed::Box<JlrsError>>
{
    unsafe {
        Module::main(&frame)
            .submodule(&frame, module)?
            .as_managed()
            .function(&frame, function)?
            .as_managed()
            //geneerinen call on olemassa myös
            .call3(&mut frame, data1, data2, data3)
            .into_jlrs_result()
            
            //result is not always i64
            //result would be Julia struct in some cases, how can we handle it? 
            //how we can use the result of the julia function in another julia function?
    }
}

pub fn _test<T>(a1: isize, a2: bool, a3: &str) {
    let mut frame = StackFrame::new(); 
    let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
    let mut julia = pending.instance(&mut frame);

    // Include some custom code defined in MyModule.jl.
    // This is safe because the included code doesn't do any strange things.
    unsafe {
        let path = PathBuf::from("structures.jl");
        //let _data = utilities::_generate_data();
        if path.exists() {
            julia.include(path).expect("Could not include file");
        } else {
            julia
                .include("src/Predicer/src/structures.jl")
                .expect("Could not include file");
        }
    }

    // An extended target provides a target for the result we want to return and a frame for
    // temporary data.
    let _x = julia.scope(|mut frame| {
        //let data = julia_interface::_to_ordered_dict(frame.as_extended_target(), &utilities::_generate_data()).unwrap();
        let data1 = Value::new(&mut frame, a1); 
        let data2 = Value::new(&mut frame, a2); //JuliaString, data->managed->string
        let data3 = JuliaString::new(&mut frame, a3).as_value(); 
        

            
            let module = "Structures";
            let function = "Test2"; 
            let _y = _call_julia_function3(frame, module, function, data1, data2, data3);
            Ok(())


    }).expect("result is an error"); 
}


pub fn _node<T>(name: &str, is_commodity: bool, is_market: bool) {
    let mut frame = StackFrame::new(); 
    let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
    let mut julia = pending.instance(&mut frame);

    // Include some custom code defined in MyModule.jl.
    // This is safe because the included code doesn't do any strange things.
    unsafe {
        let path = PathBuf::from("structures.jl");
        if path.exists() {
            julia.include(path).expect("Could not include file");
        } else {
            julia
                .include("src/Predicer/src/structures.jl")
                .expect("Could not include file");
        }
    }

    // An extended target provides a target for the result we want to return and a frame for
    // temporary data.
    let _x = julia.scope(|mut frame| {

        let n_name = JuliaString::new(&mut frame, name).as_value(); 
        let n_commodity = Value::new(&mut frame, is_commodity); 
        let n_market = Value::new(&mut frame, is_market); 
        

            
        let module = "Structures"; 
        let function = "create_node"; 
        let _result_node = _call_julia_function3(frame, module, function, n_name, n_commodity, n_market);
        Ok(())

        //Convert node to commodity if is_commodity is true  
        
    }).expect("result is an error");

}

pub fn _node2<T>(name: &str, is_commodity: bool, is_market: bool) {
    let mut frame = StackFrame::new(); 
    let mut pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };
    let mut julia = pending.instance(&mut frame);

    // Include some custom code defined in MyModule.jl.
    // This is safe because the included code doesn't do any strange things.
    unsafe {
        let path = PathBuf::from("structures.jl");
        if path.exists() {
            julia.include(path).expect("Could not include file");
        } else {
            julia
                .include("src/Predicer/src/structures.jl")
                .expect("Could not include file");
        }
    }

    // An extended target provides a target for the result we want to return and a frame for
    // temporary data.
    let _x = julia.scope(|mut frame| {

        let n_name = JuliaString::new(&mut frame, name).as_value(); 
        let n_commodity = Value::new(&mut frame, is_commodity); 
        let n_market = Value::new(&mut frame, is_market); 
        

            
        let module = "Structures"; 
        let function = "create_node"; 
        let _result_node = _call_julia_function3(frame, module, function, n_name, n_commodity, n_market).unwrap();
        

        //Convert node to commodity if is_commodity is true

        if is_commodity {

            let convert_function = "convert_to_commodity";
            let convert_result = _call_julia_function1(frame, module, convert_function, _result_node);

        }

         Ok(())
        
    }).expect("result is an error");

}


