use core::error;
use std::{io::Read, process::Command};

use tun::Device;

static OS: &str = std::env::consts::OS;


mod auth;

fn main() {

    

    let compatible=Compatibility();
    if compatible {
        CLI_controller();
    }else {
        return;
    }

   
}

fn Compatibility() -> bool{
    if OS=="linux" || OS=="windows"{
        return true
    }else{
        return false
    }
    
}

struct interface{
    interface_name:String,
    device:Device
}

fn CLI_controller(){
    println!("Starting CLI vpn");
    let mut choice=String::new();
    let mut interfaces_array:Vec<interface>=Vec::new();

    let mut input:String=String::new();
    loop {
        CLI_printer();
        choice.clear();
        input.clear();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1"=>{
                
                match CreateVirtualInterface_linux() {
                    Ok(device)=>{
                        let inter=interface{
                            interface_name: input.to_string(),
                            device:device
                        };
                        interfaces_array.push(inter);
                        
                        ()
                    },
                    Err(e)=>{
                        println!("Failed to create a interafce ");
                        println!("{e}");
                    }
                }
            },
            "2"=>
            {println!("Name of the interface");
                std::io::stdin().read_line(&mut input).unwrap();
                StartInterface_linux(input.trim());
            }
            "3"=>GetAllinterfaces_linux(),
            "4"=>{println!("Name of the interface");
                std::io::stdin().read_line(&mut input).unwrap();
                StopInterface_linux(input.trim());
            }
            "5"=>{println!("Name of the interface");
                std::io::stdin().read_line(&mut input).unwrap();
                DeleteInterafce_linux(input.trim());
            }
            "6"=>AssignIP(),
            "7"=>AddtoNetworkRules(),
            "8"=>ShowIProute(),
            "9"=>ShowRules(),
            "10"=>RemoveRule(),
            _=>println!("invalid")
        }
        
    }
}

fn CLI_printer(){
    
    println!("Choose a option");
    println!("1-Create a interafce");
    println!("2-Start a interafce");
    println!("3-Get all interafce");
    println!("4-Stop a interafce");
    println!("5-Delete a interafce");

    println!("\n");

    println!("6-Assign ip to a interface");
    println!("7-Add netorking rule for interface");
    println!("8-Show ip of th interface");
    println!("9-Show rules");
    println!("10-Remove rule for interface");

}


fn CreateVirtualInterface_linux()->Result<Device,Box<dyn std::error::Error>>{
    let mut config=tun::Configuration::default();

    
    
    
    config.address((10, 200, 100, 1));
    config.netmask((255, 255, 255, 0));
    config.up();

    let interface: tun::Device=tun::create(&config)?;
    println!("Created a interface");

    Ok(interface)
}

fn GetAllinterfaces_linux(){
    let output= Command::new("ip")
        .arg("link")
        .output()
        .expect("failed to execute");
    match output.status.success() {
        true => {
            
            println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        }
        false => {
            println!("FAILED");
            println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}

fn DeleteInterafce_linux(name:&str){
    let output=Command::new("ip")
        .args(["link","delete",name])
        .output()
        .expect("failed to delte");

    match output.status.success() {
        true => {
            println!("Interface deleted successfully");
            
        }
        false => {
            println!("FAILED to delete interface");
            println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
            
        }
    }
}

fn StopInterface_linux(name:&str){
    let output = Command::new("ip")
        .args(["link", "set", name, "down"])
        .output()
        .expect("failed");

    if output.status.success() {
        println!("Interface stopped (down)");
    } else {
        println!("FAILED to stop interface");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn StartInterface_linux(name:&str){
    let output = Command::new("ip")
        .args(["link", "set", name, "up"])
        .output()
        .expect("failed");

    if output.status.success() {
        println!("Interface started (up)");
    } else {
        println!("FAILED to start interface");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}


fn AssignIP(){
    let mut name=String::new();
    println!("Name of the interface");
    std::io::stdin().read_line( &mut name).unwrap();
    
    let mut input:String=String::new();
    println!("Enter the ip to be assinged : x.x.x.x/24 or 10.0.0.1/24");
    std::io::stdin().read_line(&mut input).unwrap();
    let output=Command::new("ip")
        .args(["addr","add",input.trim(),"dev",name.trim()])
        .output()
        .expect("failed to assing ip");

    match output.status.success() {
        true=>{

            println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        }
        false=>{
            println!("FAILED");
            println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));

        }
    }
}

fn AddtoNetworkRules(){
    let mut name=String::new();
    println!("Name of the interface");
    std::io::stdin().read_line( &mut name).unwrap();

    let mut input:String=String::new();
    println!("Enter the ip of the interafec so traffic get routed  : x.x.x.x/24 or 10.0.0.1/24");
    std::io::stdin().read_line(&mut input).unwrap();
    let output=Command::new("ip")
        .args(["route","add",input.trim(),"dev",name.trim()])
        .output()
        .expect("failed to add to rule");

    match output.status.success() {
        true=>{

            println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        }
        false=>{
            println!("FAILED");
            println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));

        }
    }
}

fn ShowIProute(){
    
    let mut input:String=String::new();
    println!("Enter the interface name to see the ip");
    std::io::stdin().read_line(&mut input).unwrap();
    let output=Command::new("ip")
        .args(["a","show",input.trim()])
        .output()
        .expect("failed to add to rule");

    match output.status.success() {
        true=>{

            println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        }
        false=>{
            println!("FAILED");
            println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));

        }
    }
}

fn ShowRules(){
    let output=Command::new("ip")
        .args(["route"])
        .output()
        .expect("failed to add to rule");

    match output.status.success() {
        true=>{

            println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        }
        false=>{
            println!("FAILED");
            println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));

        }
    }
}

fn RemoveRule(){
    let mut name=String::new();
    println!("Name of the interface");
    std::io::stdin().read_line( &mut name).unwrap();

    let mut input:String=String::new();
    println!("Enter the ip of the interafec to remove  : x.x.x.x/24");
    std::io::stdin().read_line(&mut input).unwrap();
    let output=Command::new("ip")
        .args(["route","add",input.trim(),"dev",name.trim()])
        .output()
        .expect("failed to add to rule");

    match output.status.success() {
        true=>{

            println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        }
        false=>{
            println!("FAILED");
            println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));

        }
    }
}



fn ReadPacket( dev: &mut Device)-> Result<(), Box<dyn std::error::Error>>{
    let mut buf = vec![0u8; 1500];
    loop {
        let byte_packet=dev.read(&mut buf)?;

        let valid_packet=&buf[..byte_packet];


        let ip_protocol_version=valid_packet[0] >>4;
        
        IP_protocol(&ip_protocol_version);

        if ip_protocol_version==4{
            let ihl = valid_packet[0] & 0x0F;
            let ip_header_len = (ihl * 4);
            transport_protocol(&valid_packet,&valid_packet[9],&ip_header_len);
            
        }

        if ip_protocol_version==6{

        }
    }
    

    Ok(())
}

fn IP_protocol(version:&u8){
    match version {
        4=>{
            println!("IP protocol is {version}");
            
        }
        6=>{
            println!("IP protocol is {version}");
            
        }
        _=>{
            println!("not a valid IP protocol");
        
        }
    }
}

fn transport_protocol(packet:&[u8],number :&u8,transport_start:&u8){
    match number {
        1=>{
            println!("Transport protocol is ICMP");
        }
        6=>{
            println!("Transport protocol is TCP");
            
        }
        17=>{
            println!("Transport protocol is UDP");
        }
        _=>{
            println!("invalid protocol")
        }
    }
}