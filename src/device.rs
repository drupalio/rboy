use cpu::CPU;
use keypad::KeypadKey;
use printer::GbPrinter;

pub struct Device {
    cpu: CPU<'static>,
}

fn stdoutprinter(v: u8) -> Option<u8> {
    use std::io::Write;

    print!("{}", v as char);
    let _ = ::std::io::stdout().flush();

    None
}

impl Device {
    pub fn new(romname: &str) -> ::StrResult<Device> {
        CPU::new(romname, None).map(|cpu| Device { cpu: cpu })
    }

    pub fn new_cgb(romname: &str) -> ::StrResult<Device> {
        CPU::new_cgb(romname, None).map(|cpu| Device { cpu: cpu })
    }

    pub fn do_cycle(&mut self) -> u32 {
        self.cpu.do_cycle()
    }

    pub fn set_stdout(&mut self, output: bool) {
        if output {
            self.cpu.mmu.serial.set_callback(Box::new(stdoutprinter));
        }
        else {
            self.cpu.mmu.serial.unset_callback();
        }
    }

    pub fn attach_printer(&mut self) {
        let mut printer = GbPrinter::new();

        let printfun = move |v: u8| -> Option<u8> {
            Some(printer.send(v))
        };

        self.cpu.mmu.serial.set_callback(Box::new(printfun));
    }

    pub fn check_and_reset_gpu_updated(&mut self) -> bool {
        let result = self.cpu.mmu.gpu.updated;
        self.cpu.mmu.gpu.updated = false;
        result
    }

    pub fn get_gpu_data(&self) -> &[u8] {
        &self.cpu.mmu.gpu.data
    }

    pub fn enable_audio(&mut self, player: Box<::sound::AudioPlayer>) {
        self.cpu.mmu.sound = Some(::sound::Sound::new(player));
    }

    pub fn sync_audio(&mut self) {
        if let Some(ref mut sound) = self.cpu.mmu.sound {
            sound.sync();
        }
    }

    pub fn keyup(&mut self, key: KeypadKey) {
        self.cpu.mmu.keypad.keyup(key);
    }

    pub fn keydown(&mut self, key: KeypadKey) {
        self.cpu.mmu.keypad.keydown(key);
    }

    pub fn romname(&self) -> String {
        self.cpu.mmu.mbc.romname()
    }
}
