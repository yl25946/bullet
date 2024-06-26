use super::DeviceHandles;

pub unsafe fn sigmoid_mpe(
    handle: &DeviceHandles,
    buffer_size: usize,
    outputs: *mut f32,
    results: *const f32,
    errors: *mut f32,
    power: f32,
) {
    let results = results as usize;
    let outputs = outputs as usize;
    let errors = errors as usize;

    handle.split_workload(buffer_size, |thread, idx| {
        let this_result = (results as *const f32).add(idx);
        let this_output = (outputs as *mut f32).add(idx);
        let this_error = (errors as *mut f32).add(thread);

        let result = *this_result;
        let output = *this_output;

        let sigmoid = 1.0 / (1.0 + (-output).exp());
        let diff = sigmoid - result;
        let absd = diff.abs();

        *this_output = diff.signum() * absd.powf(power - 1.0) * sigmoid * (1.0 - sigmoid);
        *this_error += absd.powf(power);
    });
}
