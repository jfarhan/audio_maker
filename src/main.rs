use byteorder::LittleEndian;
use std::io;
use std::io::{Read,Write};
use std::path::Path;
use std::fs::File;
use std::env;

extern crate byteorder;
use byteorder::{BigEndian,WriteBytesExt};
use test2::audio_test;
fn main(){
    let mut count=0;
    let sampleRate=44100.0;
    let freq=100.0/7000.0;                                                           //Frequency in hz
    /*
    =========================================================================================================
            When the frequency is 1.0 Hz , I get 7kHz for some reason.
            So adjust accordingly.
    =========================================================================================================
    */
    let duration=2.0;                                                       //Duration of the wave in seconds
    let mut  testwave =audio_test::Wave::new(1,1,sampleRate as u32,32);
    let nsamples=(duration*sampleRate) as u32;
    testwave.set_duration(duration);
    let mut music_vec:Vec<f32>=Vec::new();


    //Input sine wave into the .wav file

    for time in 0..nsamples{
        let mut valvec=Vec::new();
        let val=(freq*(time as f32)).sin()   ;
        /*
        Here,input whatever function you want  as a function of time,which acts as time here.
        so val=sin(freq*time)

        */
        valvec.push(val);
        testwave.add_wave_sample(valvec);
        count+=4;
        println!("count is {:?}",count );
    }

    println!("music vec is {:?}",music_vec );

    //testwave.add_wave_sample(music_vec);
    testwave.wave_to_file();


}

pub fn create_header_for_write()->Vec<u8>{

    //experimentation
    let mut header_vec=Vec::new();
    let p:[u8;4]=[b'R',b'I',b'F',b'F'];

    for m in p.iter(){
        header_vec.push(*m)
    }

    /*
    let mut header_vec=Vec::new();
    header_vec.push(b'R');
    header_vec.push(b'I');
    header_vec.push(b'F');
    header_vec.push(b'F');
    header_vec.write_u32::<LittleEndian>(453).unwrap();



*/
    header_vec
}


fn mani(){
    let p=create_header_for_write();
    println!("{:?}",p );
}
//endianness figured out






fn ma(){

    let mut newfile=File::create("new").expect("Couldn't do this");
    newfile.write(&test());
    let mut b:Vec<u8>=Vec::new();
    b=b"Hello world".to_vec();


}

pub fn test()->Vec<u8>{

    let mut new_vec:Vec<u8>=Vec::new();
    let p = "RIFF----WAVEfmt ".as_bytes();
    for n in p{
    new_vec.push(*n);
    }
//    new_vec.push()



//    new_vec.push();

    new_vec

}
/*

Don't do anything to this

*/
fn mainok() {
    let args:Vec<String>=env::args().collect();
    let filename=&args[1];
    let path =Path::new(filename);
    println!("{:?}",path );
    let mut img=file_to_bytes(path).unwrap();

    let  length=img.len();
    for m in 0..length {
        if img[m]==76{
            img[m]=200;
        }
    }

    let mut new_image=File::create("new.png").expect("Couldn't");
    new_image.write(&img);



    println!("{:?}",img );
}

fn file_to_bytes(path:&Path)->Result<Vec<u8>,std::io::Error>{

File::open(path).and_then(|mut file|{
    let mut bytes=Vec::new();
    (file.read_to_end(&mut bytes))?;

    Ok(bytes)

})

}
