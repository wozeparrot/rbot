use rbotlib::*;

fn main() {
    let robot = rbotlib::robot_base::RobotBase::new(500).expect("HAL Failed to Init");
    robot.run();

    let ds = robot.init_ds();

    let joystick_port = driverstation::JoystickPort::new(0).unwrap();
    let joystick_axis = driverstation::JoystickAxis::new(1).unwrap();

    loop {
        println!("joystick: {}", ds.get_stick_axis(joystick_port, joystick_axis).unwrap());
        println!("time: {}", fpga::get_time_us().unwrap());
    }
}