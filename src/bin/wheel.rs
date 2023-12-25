use zenoh::{
    config::Config,
    prelude::r#async::*,
};

use async_std;
use zenoh_interface::{CmdVel, motor_controll::QuadMotor};


#[async_std::main]
async fn main()
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let sub_topic = "cmd_vel".to_string();
    let pub_topic = "wheel_command".to_string();

    let subscriber = session.declare_subscriber(&sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(&pub_topic).res().await.unwrap();

    let diagonal = ((2.0_f32).sqrt() / 2.0) as f32;


    loop {
        let get_data = subscriber.recv_async().await.unwrap();

        let deserialized:CmdVel = serde_json::from_str(&get_data.value.to_string()).unwrap();

        let wheel_cmd = QuadMotor{
            power_0:deserialized.x*diagonal - deserialized.y*diagonal + deserialized.rotation_power,
            power_1:deserialized.x*diagonal - deserialized.y*diagonal + deserialized.rotation_power,
            power_2:-deserialized.x*diagonal - deserialized.y*diagonal + deserialized.rotation_power,
            power_3:-deserialized.x*diagonal - deserialized.y*diagonal + deserialized.rotation_power,
        };

        println!("fl:{}, fr:{}, rl:{}, rr:{}", wheel_cmd.power_0, wheel_cmd.power_1, wheel_cmd.power_2, wheel_cmd.power_3);

        let serialized = serde_json::to_string(&wheel_cmd).unwrap();

        publisher.put(serialized).res().await.unwrap();
    }
}


// wheel
// 0    1
// 2    3