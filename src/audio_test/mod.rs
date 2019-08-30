//First test with the wav file encoding thingy
//                          --cubeasauros
/*

before making the file do the

write_to_file() function








*/

extern crate byteorder;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use byteorder::{LittleEndian,WriteBytesExt};





pub struct WaveHeader{

    //Header chunk
    riff_identifier:[u8;4],        //contains RIFF and thingies
    chunksize:u32,
    wave_identifier:[u8;4],

    //Format
    format_identifier:[u8;4],
    format_subchunk_length:u32,
    format_type:u16,
    channels:u16,
    sample_rate:u32,
    bytes_per_second:u32,
    block_align:u16,
    bits_per_sample:u16,

    //audio data
    data_identifier:[u8;4],
    audio_subchunk_length:u32,
    //24 bit sample value needed here;

}

pub struct Wave{

    header:WaveHeader,

    data:Vec<u8>,

    index:u64,

    size:u64,

    nsamples:u64,   //number of samples

}


impl Wave{
    pub fn new(fmtType:u16,numChannels:u16,sampleRate:u32,bitsPerSample:u16)->Wave{
        Wave{
            header:WaveHeader::new(fmtType,numChannels,sampleRate,bitsPerSample),
            data:Vec::new(),
            index:0,
            size:0,
            nsamples:0
        }
    }

    pub fn set_duration(&mut self,seconds:f64){
        let total_bytes:u64=((self.header.bytes_per_second as f64)*seconds) as u64;
        self.size=total_bytes;
        self.nsamples=(((self.header.channels as u32*self.header.sample_rate) as f64)*seconds) as u64;
        self.header.chunksize=4+8+16+8+(total_bytes as u32);
        self.header.audio_subchunk_length=total_bytes as u32;
        self.header.format_subchunk_length=16;
    }

    pub fn add_wave_sample(&mut self,samples:Vec<f32>){
        //i
        //sample8bit u16
        //sample16bit
        //

        /*for i in 0..numChannels{
        sample8bit=(short int)(127+127.0*samples[i])
        tolittleendian
        sample=(char*)&sample8bit
        wave.data[wave.index]=sample[0]


        Ice box for today:
                finish this.
        }*/

        //This could be kinda right , probably , idk :}
        //This is probably wrong
        /*
        proposed solution:


        */
        if self.header.bits_per_sample==8{
            for i in 0..self.header.channels as usize {
                let sample8bit=(127.0+127.0*samples[i]) as u16 ;
                let mut p:Vec<u8>=Vec::new();
                p.write_u16::<LittleEndian>(sample8bit).unwrap();
                let sample=p[0] ;
                self.data[self.index as usize]=sample;
                self.index=self.index+1;
            }
        }
        //I am gonna test this!
        //hope it works :}






        //This is kinda wrong too
        if self.header.bits_per_sample==16{
            for i in 0..self.header.channels as usize {
                let sample16bit=(32767.0*samples[i]) as u32;

                let mut sample:Vec<u8>=Vec::new();
                sample.write_u32::<LittleEndian>(sample16bit);
                self.data[self.index as usize]=sample[0];
                self.data[(self.index+1) as usize]=sample[1];
                self.index+=2;
            }
        }


        //This is all wrong , I gotta find a better solution to this
        if self.header.bits_per_sample==32{
            for i in 0..self.header.channels as usize {
                let sample16bit=((2.0f32.powi(32-1)-1.0)*samples[i]) as u32;
                //The little endian thingy
                let mut sample:Vec<u8>=Vec::new();
                println!("something works");
                println!("Data length = {:?}",self.data.len());
                sample.write_u32::<LittleEndian>(sample16bit).unwrap();
                self.data.push(sample[0]);               //[self.index as usize]=sample[0];
                self.data.push(sample[1]);//[self.index as usize + 1]=sample[1];
                self.data.push(sample[2]);//[self.index as usize +1]=sample[2];
                self.data.push(sample[3]);//[self.index as usize + 1]=sample[3];
                self.index=self.index+4;
            }
        }




    }

    pub fn wave_to_file(&mut self){
        let mut file=File::create("test2.wav").expect("Couldn't create a file");
        //edit this shit
        //file.write(&self.create_header_for_write())
        let mut audfile=self.create_header_for_write();
        for  m in self.data.iter(){
            audfile.push(*m);
        }
        println!("{:?}",audfile );
        file.write(&audfile);

        //file.write()
    }



//SOME PARTS OF STUFF DONE
//KINDA

//too boilerplatey
    pub fn create_header_for_write(&mut self)->Vec<u8>{

        let mut header_vec=Vec::new();
        //RIFF
        for m in self.header.riff_identifier.iter(){
                header_vec.push(*m);
        }

        //Format subchunk length
        header_vec.write_u32::<LittleEndian>(self.header.chunksize).unwrap();

        //  WAVE
        for m in self.header.wave_identifier.iter(){
            header_vec.push(*m);
        }

        //format identifier

        for m in self.header.format_identifier.iter(){
            header_vec.push(*m)
        }


        //format subchunk length
        header_vec.write_u32::<LittleEndian>(self.header.audio_subchunk_length).unwrap();

        //format type
        header_vec.write_u16::<LittleEndian>(self.header.format_type).unwrap();

        //channels
        header_vec.write_u16::<LittleEndian>(self.header.channels).unwrap();

        //sample_rate

        header_vec.write_u32::<LittleEndian>(self.header.sample_rate).unwrap();

        //bytes per second
        header_vec.write_u32::<LittleEndian>(self.header.bytes_per_second).unwrap();

        //block_align
        header_vec.write_u16::<LittleEndian>(self.header.block_align).unwrap();

        //bits per sample
        header_vec.write_u16::<LittleEndian>(self.header.bits_per_sample).unwrap();

        //data_identifier
        for m in self.header.data_identifier.iter(){
            header_vec.push(*m);
        }

        //audio_subchunk_length
        header_vec.write_u32::<LittleEndian>(self.header.audio_subchunk_length).unwrap();


        header_vec
    }






}




impl WaveHeader {
    pub fn new(fmtType:u16,numChannels:u16,sampleRate:u32,bitsPerSample:u16,)->WaveHeader{
        WaveHeader{
        //header
        riff_identifier:[0x52,0x49,0x46,0x46],
        chunksize:36,        //by default
        wave_identifier:[0x57,0x41,0x56,0x45],

        //format
        format_identifier:[0x66,0x6d,0x74,0x20],
        format_subchunk_length:16,
        format_type:fmtType,
        channels:numChannels,
        sample_rate:sampleRate,
        bytes_per_second:sampleRate*(numChannels as u32)*(bitsPerSample as u32)/8,
        block_align:bitsPerSample*numChannels/8,
        bits_per_sample:bitsPerSample,

        //audio_data
        data_identifier:[0x64,0x61,0x74,0x61],
        audio_subchunk_length:0,

    }



}





}
