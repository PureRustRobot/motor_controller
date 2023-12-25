use zenoh::{
    config::Config,
    prelude::r#async::*,
};

use async_std;
use zenoh_interface::{CmdVel, motor_controll::SingleMotor};


#[async_std::main]
async fn main()
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let sub_topic = "cmd_vel".to_string();
    let pub_topic = "single_command".to_string();

    let subscriber = session.declare_subscriber(&sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(&pub_topic).res().await.unwrap();


    loop {
        let get_data = subscriber.recv_async().await.unwrap();

        let deserialized:CmdVel = serde_json::from_str(&get_data.value.to_string()).unwrap();

        let cmd = SingleMotor{
            power:deserialized.rotation_power,
        };

        let serialized = serde_json::to_string(&cmd).unwrap();

        publisher.put(serialized).res().await.unwrap();
    }
}


// wheel
// 0    1
// 2    3