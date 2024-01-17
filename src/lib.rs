use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use prr_msgs::msg::*;
use zenoh_manage_utils::param::{get_str_param, get_bool_param};
use zenoh_manage_utils::logger;


pub async fn wheel_controller(
    node_name:&str, 
    sub_topic:&str,
    pub_topic:&str,
)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let subscriber = session.declare_subscriber(&sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(&pub_topic).res().await.unwrap();

    let diagonal = ((2.0_f32).sqrt() / 2.0) as f32;

    let msg = format!("Start sub:{}, pub:{}", subscriber.key_expr().to_string(), publisher.key_expr().to_string());
    logger::log_info(node_name, msg);


    loop {
        let sample = subscriber.recv_async().await.unwrap();

        let get_data = deserialize_cmdvel(sample.value.to_string());

        let wheel_cmd = Wheel{
            front_left:get_data.x*diagonal - get_data.y*diagonal + get_data.rotation_power,
            front_right:get_data.x*diagonal - get_data.y*diagonal + get_data.rotation_power,
            rear_left:-get_data.x*diagonal - get_data.y*diagonal + get_data.rotation_power,
            rear_right:-get_data.x*diagonal - get_data.y*diagonal + get_data.rotation_power,
        };

        let serialized = serialize_wheel(&wheel_cmd);

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

        let get_data:CmdVel = serde_json::from_str(&get_data.value.to_string()).unwrap();

        let mut cmd = SingleMotor{
            power:get_data.rotation_power,
        };

        if reversal
        {
            cmd.power *= -1.0;
        }

        let serialized = serde_json::to_string(&cmd).unwrap();

        publisher.put(serialized).res().await.unwrap();
    }
}
