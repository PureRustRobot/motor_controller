use async_std;
use motor_controller::*;
use zenoh::Error;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let wheel_task = async_std::task::spawn(wheel_converter("./param/wheel.yaml"));

    wheel_task.await?;

    Ok(())
}


// wheel
// 0    1
// 2    3