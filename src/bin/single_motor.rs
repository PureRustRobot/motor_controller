use zenoh::Error;

use async_std;

use motor_controller::single_motor_to_single_motor;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let task = async_std::task::spawn(single_motor_to_single_motor("updown_controller", "cmd/updown", "motor/updown", false, 1.0));

    task.await?;

    Ok(())
}