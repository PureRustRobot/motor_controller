use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use zenoh_interface::{CmdVel, motor_controll::{QuadMotor, SingleMotor}};
use zenoh_manage_utils::param::{get_str_param, get_bool_param};
use zenoh_manage_utils::logger;


pub async fn wheel_controller(node_name:&str, yaml_path:&str)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let sub_topic = get_str_param(yaml_path, node_name, "sub_topic", "cmd_vel".to_string());
    let pub_topic = get_str_param(yaml_path, node_name, "pub_topic", "output".to_string());

    let subscriber = session.declare_subscriber(&sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(&pub_topic).res().await.unwrap();

    let diagonal = ((2.0_f32).sqrt() / 2.0) as f32;

    let msg = format!("Start sub:{}, pub:{}", subscriber.key_expr().to_string(), publisher.key_expr().to_string());
    logger::log_info(node_name, msg);


    loop {
        let get_data = subscriber.recv_async().await.unwrap();

        let deserialized:CmdVel = serde_json::from_str(&get_data.value.to_string()).unwrap();

        let wheel_cmd = QuadMotor{
            power_0:deserialized.x*diagonal - deserialized.y*diagonal + deserialized.rotation_power,
            power_1:deserialized.x*diagonal - deserialized.y*diagonal + deserialized.rotation_power,
            power_2:-deserialized.x*diagonal - deserialized.y*diagonal + deserialized.rotation_power,
            power_3:-deserialized.x*diagonal - deserialized.y*diagonal + deserialized.rotation_power,
        };

        let serialized = serde_json::to_string(&wheel_cmd).unwrap();
        let log_data = format!("send :{}", serialized);

        logger::log_info(node_name, log_data);

        publisher.put(serialized).res().await.unwrap();
    }
}

pub async fn single_controller(node_name:&str, yaml_path:&str)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let sub_topic = get_str_param(yaml_path, node_name, "sub_topic", "input".to_string());
    let pub_topic = get_str_param(yaml_path, node_name, "pub_topic", "input".to_string());
    let reversal = get_bool_param(yaml_path, node_name, "reversal", false);

    let subscriber = session.declare_subscriber(&sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(&pub_topic).res().await.unwrap();

    let msg = format!("Start sub:{}, pub:{}", subscriber.key_expr().to_string(), publisher.key_expr().to_string());
    logger::log_info(node_name, msg);

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
