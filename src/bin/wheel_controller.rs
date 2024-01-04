use zenoh::Error;

use async_std;

use motor_controller::wheel_controller;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let task = async_std::task::spawn(wheel_controller("wheel_controller", "./param/wheel.yaml"));

    task.await?;

    Ok(())
}


// wheel
// 0    1
// 2    3