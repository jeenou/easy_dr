pub mod julia {
    use jlrs::prelude::*;
    use jlrs::data::managed::value::ValueResult;
    use jlrs::prelude::*;
    use jlrs::data::managed::value::ValueData;
    use jlrs::memory::target::ExtendedTarget;
    use jlrs::data::managed::union_all::UnionAll;
    use jlrs::data::managed::function::Function;

    fn prepare_callable<'target, 'data, T: Target<'target>>(target: T, function: &[&str]) -> Function<'target, 'data> {
        unsafe {
            let mut module = Module::main(&target);
            for submodule in &function[0..function.len()-1] {
                module = module.submodule(&target, submodule).expect(&format!("error with module {}", &submodule)).as_managed();
            }
            let function_name = function.last().expect("function name missing");
            module.function(&target, function_name).expect(&format!("error with function {}", &function_name)).as_managed()
        }
    }

     pub fn call<'target, 'data, T: Target<'target>>(
        target: T,
        function: &[&str],
        args: &[Value<'_, 'data>],
    ) -> ValueResult<'target, 'data, T> {
        let callable = prepare_callable(&target, &function);
        unsafe {
            if args.len() == 0 {
                callable.call0(target)
            } else {
                callable.call(target, args)
            }
        }
    }

    // Convert a slice of pairs of strings and i32's to an `OrderedDict`
    pub fn to_ordered_dict<'target, T>(
        target: ExtendedTarget<'target, '_, '_, T>,
        data: &[(String, f64)],
    ) -> JlrsResult<ValueData<'target, 'static, T>>
    where
        T: Target<'target>,
    {
        // An extended target provides a target for the result we want to return and a frame for
        // temporary data.
        let (target, frame) = target.split();
        frame.scope(|mut frame| {
            // Get OrderedDict, load OrderedCollections if it can't be found. An error is returned if
            // OrderedCollections hasn't been installed yet.
            // OrderedDict is a UnionAll because it has type parameters that must be set
            let ordered_dict = Module::main(&frame).global(&mut frame, "OrderedDict");
            let ordered_dict_ua = match ordered_dict {
                Ok(ordered_dict) => ordered_dict,
                Err(_) => {
                    // Safety: using this package is fine.
                    unsafe {
                        Value::eval_string(&mut frame, "using OrderedCollections")
                            .into_jlrs_result()?
                    };
                    Module::main(&frame).global(&mut frame, "OrderedDict")?
                }
            }
            .cast::<UnionAll>()?;
            // The key and value type.
            let types = [
                DataType::string_type(&frame).as_value(),
                DataType::float64_type(&frame).as_value(),
            ];
            // Apply the types to the OrderedDict UnionAll to create the OrderedDict{String, Int32}
            // DataType, and call its constructor.
            //
            // Safety: the types are correct and the constructor doesn't access any data that might
            // be in use.
            let ordered_dict = unsafe {
                let ordered_dict_ty = ordered_dict_ua
                    .apply_types(&mut frame, types)
                    .into_jlrs_result()?;
                ordered_dict_ty.call0(&mut frame).into_jlrs_result()?
            };
            let setindex_fn = Module::base(&target).function(&mut frame, "setindex!")?;
            for (key, value) in data {
                // Create the keys and values in temporary scopes to avoid rooting an arbitrarily
                // large number of pairs in the current frame.
                frame.scope(|mut frame| {
                    let key = JuliaString::new(&mut frame, key).as_value();
                    let value = Value::new(&mut frame, *value);
                    // Safety: the ordered dict can only be used in this function until it is
                    // returned, setindex! is a safe function.
                    unsafe {
                        setindex_fn
                            .call3(&mut frame, ordered_dict, value, key)
                            .into_jlrs_result()?;
                    }
                    Ok(())
                })?;
            }
            Ok(ordered_dict.root(target))
        })
    }

}
