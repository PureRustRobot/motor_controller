use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use zenoh_interface::{CmdVel, motor_controll::{QuadMotor, SingleMotor}};
use zenoh_manage_utils::param::{get_str_param, get_bool_param};


pub async fn wheel_converter(yaml_path:&str)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let sub_topic = get_str_param(yaml_path, "wheel", "sub_topic", "cmd_vel".to_string());
    let pub_topic = get_str_param(yaml_path, "wheel", "pub_topic", "output".to_string());

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

pub async fn single_motor(yaml_path:&str)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let sub_topic = get_str_param(yaml_path, "single_motor", "sub_topic", "input".to_string());
    let pub_topic = get_str_param(yaml_path, "single_motor", "pub_topic", "input".to_string());
    let reversal = get_bool_param(yaml_path, "single_motor", "reversal", false);

    let subscriber = session.declare_subscriber(&sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(&pub_topic).res().await.unwrap();


    loop {
        let get_data = subscriber.recv_async().await.unwrap();

        let deserialized:CmdVel = serde_json::from_str(&get_data.value.to_string()).unwrap();

        let mut cmd = SingleMotor{
            power:deserialized.rotation_power,
        };

        if reversal
        {
            cmd.power *= -1.0;
        }

        let serialized = serde_json::to_string(&cmd).unwrap();

        publisher.put(serialized).res().await.unwrap();
    }
}
