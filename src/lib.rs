use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use prr_msgs::msg::*;
use zenoh_manage_utils::logger;


pub async fn cmd_vel_to_wheel(
    node_name:&str, 
    sub_topic:&str,
    pub_topic:&str,
    speed_rate:f32,
)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let subscriber = session.declare_subscriber(sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(pub_topic).res().await.unwrap();

    let diagonal = ((2.0_f32).sqrt() / 2.0) as f32;

    let msg = format!("Start sub:{}, pub:{}", subscriber.key_expr().to_string(), publisher.key_expr().to_string());
    logger::log_info(node_name, msg);


    loop {
        let sample = subscriber.recv_async().await.unwrap();

        let get_data = deserialize_cmdvel(sample.value.to_string());

        let mut wheel_cmd = Wheel{
            front_left:get_data.x*diagonal - get_data.y*diagonal + get_data.rotation_power,
            front_right:get_data.x*diagonal - get_data.y*diagonal + get_data.rotation_power,
            rear_left:-get_data.x*diagonal - get_data.y*diagonal + get_data.rotation_power,
            rear_right:-get_data.x*diagonal - get_data.y*diagonal + get_data.rotation_power,
        };

        wheel_cmd.front_left = wheel_cmd.front_left * speed_rate;
        wheel_cmd.front_right = wheel_cmd.front_right * speed_rate;
        wheel_cmd.rear_left = wheel_cmd.rear_left * speed_rate;
        wheel_cmd.rear_right = wheel_cmd.rear_right * speed_rate;

        let serialized = serialize_wheel(&wheel_cmd);

        let log_data = format!("send :{}", serialized);

        logger::log_info(node_name, log_data);

        publisher.put(serialized).res().await.unwrap();
    }
}

pub async fn single_motor_to_single_motor(
    node_name:&str, 
    sub_topic:&str,
    pub_topic:&str,
    enable_reversal:bool,
    speed_rate:f32,
)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let subscriber = session.declare_subscriber(sub_topic).res().await.unwrap();
    let publisher = session.declare_publisher(pub_topic).res().await.unwrap();

    let msg = format!("Start sub:{}, pub:{}", subscriber.key_expr().to_string(), publisher.key_expr().to_string());
    logger::log_info(node_name, msg);

    loop {
        let sample = subscriber.recv_async().await.unwrap();

        let mut get_data = deserialize_singlemotor(sample.value.to_string());

        get_data.power = get_data.power * speed_rate;

        if enable_reversal
        {
            get_data.power = get_data.power * -1.0;
        }

        publisher.put(serialize_singlemotor(&get_data)).res().await.unwrap();
    }
}
