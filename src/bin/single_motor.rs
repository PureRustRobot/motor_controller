use zenoh::Error;

use async_std;

use motor_controller::single_controller;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let task = async_std::task::spawn(single_controller("updown_controller", "./param/single_motor.yaml"));

    task.await?;

    Ok(())
}