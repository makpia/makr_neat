pub enum neat_activation_func_types {
    null,
    step,
    sigmoid,
    relu,
}


mod _af {
    pub fn null(x: f64) -> f64 {
        x
    }

    pub fn step(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else {
            0.0
        }
    }

    pub fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn relu(x: f64) -> f64 {
        if x > 0.0 {
            x
        } else {
            0.0
        }
    }
}


pub fn neat_activation_func_calculate(
    &func: neat_activation_func_types,
    x: f64,
) -> f64 {
    match func {
        neat_activation_func_types::null => _af::null(x),
        neat_activation_func_types::step => _af::step(x),
        neat_activation_func_types::sigmoid => _af::sigmoid(x),
        neat_activation_func_types::relu => _af::relu(x),
    }
}
