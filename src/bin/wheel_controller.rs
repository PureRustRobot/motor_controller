use zenoh::Error;

use async_std;

use motor_controller::cmd_vel_to_wheel;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let task = async_std::task::spawn(cmd_vel_to_wheel("cmd_vel_to_wheel", "cmd_vel", "cmd/wheel", 1.0));

    task.await?;

    Ok(())
}


// wheel
// 0    1
// 2    3