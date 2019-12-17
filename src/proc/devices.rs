define_struct! {
    pub struct Devices {
        character: Vec<(usize, String)>,
        block: Vec<(usize, String)>,
    }
}

use std::str::FromStr;
impl FromStr for Devices {
    type Err = crate::ProcErr;

    fn from_str(s: &str) -> Result<Devices, crate::ProcErr> {
        let areas: Vec<&str> = s.split("\n\n").collect();
        if areas.len() != 2 {
            return Err(bfe!("devices consist of two parts".to_string()));
        }

        let mut character = vec![];
        let character_devices: Vec<&str> = areas[0].trim().split_ascii_whitespace().skip(2).collect();
        for s in character_devices.windows(2).step_by(2) {
            let num = s[0].parse::<usize>()?;
            let name = s[1].to_string();
            character.push((num, name));
        }

        let mut block = vec![];
        let block_devices: Vec<&str> = areas[1].trim().split_ascii_whitespace().skip(2).collect();
        
        for s in block_devices.windows(2).step_by(2) {
            let num = s[0].parse::<usize>()?;
            let name = s[1].to_string();
            block.push((num, name));
        }

        Ok(Devices{character, block})
    }
}

instance_impl! {
    devices, "/proc/devices", Devices
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_devices() {
        let source = {
"Character devices:
1 mem
4 /dev/vc/0
4 tty
4 ttyS
5 /dev/tty
5 /dev/console
5 /dev/ptmx
7 vcs
10 misc
13 input
14 sound
21 sg
29 fb
116 alsa
128 ptm
136 pts
162 raw
180 usb
188 ttyUSB
189 usb_device
202 cpu/msr
203 cpu/cpuid
226 drm
244 BaseRemoteCtl
245 aux
246 hidraw
247 usbmon
248 bsg
249 hmm_device
250 watchdog
251 iio
252 rtc
253 dax
254 tpm

Block devices:
259 blkext
8 sd
9 md
65 sd
66 sd
67 sd
68 sd
69 sd
70 sd
71 sd
128 sd
129 sd
130 sd
131 sd
132 sd
133 sd
134 sd
135 sd
254 mdp
"
        };
        let correct = Devices{
            character: vec![
                (    1, String::from("mem")),
                (    4, String::from("/dev/vc/0")),
                (    4, String::from("tty")),
                (    4, String::from("ttyS")),
                (    5, String::from("/dev/tty")),
                (    5, String::from("/dev/console")),
                (    5, String::from("/dev/ptmx")),
                (    7, String::from("vcs")),
                (   10, String::from("misc")),
                (   13, String::from("input")),
                (   14, String::from("sound")),
                (   21, String::from("sg")),
                (   29, String::from("fb")),  
                (  116, String::from("alsa")),  
                (  128, String::from("ptm")),  
                (  136, String::from("pts")),  
                (  162, String::from("raw")),  
                (  180, String::from("usb")),      
                (  188, String::from("ttyUSB")),  
                (  189, String::from("usb_device")),      
                (  202, String::from("cpu/msr")),  
                (  203, String::from("cpu/cpuid")),      
                (  226, String::from("drm")),  
                (  244, String::from("BaseRemoteCtl")),
                (  245, String::from("aux")),    
                (  246, String::from("hidraw")), 
                (  247, String::from("usbmon")), 
                (  248, String::from("bsg")),
                (  249, String::from("hmm_device")),     
                (  250, String::from("watchdog")), 
                (  251, String::from("iio")),
                (  252, String::from("rtc")),
                (  253, String::from("dax")),
                (  254, String::from("tpm")),
            ],
            block: vec![
                (259, String::from("blkext")),
                (8  , String::from("sd")),
                (9  , String::from("md")),
                (65 , String::from("sd")),
                (66 , String::from("sd")),
                (67 , String::from("sd")),
                (68 , String::from("sd")),
                (69 , String::from("sd")),
                (70 , String::from("sd")),
                (71 , String::from("sd")),
                (128, String::from("sd")),
                (129, String::from("sd")),
                (130, String::from("sd")),
                (131, String::from("sd")),
                (132, String::from("sd")),
                (133, String::from("sd")),
                (134, String::from("sd")),
                (135, String::from("sd")),
                (254, String::from("mdp")),
            ],
        };
        assert_eq!(correct, source.parse::<Devices>().unwrap());
    }
}