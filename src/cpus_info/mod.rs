use core::fmt;

use std::{
    fs::File,
    io::{Read, Seek},
};

use self::parser::{number_of_spaced_elements, parse_cpu_file_line};

mod parser;

const PATH: &str = "/proc/stat";

#[derive(Debug, Clone)]
pub struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    name: String,
    values: Vec<f64>,
}

impl Cpu {
    fn get_value<'a, 'b>(&'a mut self, buffer: &'b str, nelements: usize) -> &'b str {
        let res = parse_cpu_file_line(buffer, &mut self.values, nelements).unwrap();
        res.0
    }
}

#[derive(Debug)]
pub struct CpusInfo {
    old_cpus: Vec<Cpu>,
    new_cpus: Vec<Cpu>,
    stat_file: File,
    buffer: String,
    nelements: usize,
}

impl CpusInfo {
    
    pub fn new() -> Self {
        let mut old_cpus = Vec::new();
        let mut new_cpus = Vec::new();
        let mut buffer = String::new();
        let mut stat_file = File::open(PATH).expect("File Not Found");
        stat_file.read_to_string(&mut buffer).unwrap();
        stat_file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let size = buffer.matches("cpu").count();

        for i in 0..size {
            let name = match i {
                0 => "cpu".to_owned(),
                n @ _ => format!("cpu{}", n - 1),
            };
            old_cpus.push(Cpu {
                name: name.to_owned(),
                values: Vec::new(),
            });
            new_cpus.push(Cpu {
                name,
                values: Vec::new(),
            })
        }

        let nelements = number_of_spaced_elements(&buffer) - 1;

        let mut ret = CpusInfo {
            old_cpus,
            new_cpus,
            stat_file,
            buffer,
            nelements,
        };
        ret.get_old();
        ret.get_new();
        ret
    }

    fn read_file(&mut self) {
        let Self {
            stat_file, buffer, ..
        } = self;
        buffer.clear();
        stat_file.read_to_string(buffer).unwrap();
        stat_file.seek(std::io::SeekFrom::Start(0)).unwrap();
    }

    fn get_new(&mut self) {
        self.read_file();
        let Self {
            new_cpus,
            buffer,
            nelements,
            ..
        } = self;
        for new_cpu in new_cpus.iter_mut() {
            new_cpu.values.clear();
        }

        let mut buffer_view = buffer.as_str();

        for cpu in new_cpus {
            buffer_view = cpu.get_value(&buffer_view, *nelements);
        }
    }

    fn get_old(&mut self) {
        self.read_file();
        let Self {
            old_cpus,
            buffer,
            nelements,
            ..
        } = self;

        for old_cpu in old_cpus.iter_mut() {
            old_cpu.values.clear();
        }

        let mut buffer_view = buffer.as_str();

        for cpu in old_cpus {
            buffer_view = cpu.get_value(&buffer_view, *nelements);
        }
    }

    fn print_delta(&self) {
        let Self {
            old_cpus, new_cpus, ..
        } = self;

        let mut elemnts_iter = old_cpus.iter().zip(new_cpus).peekable();
        while let Some((old, new)) = elemnts_iter.next() {
            assert_eq!(old.name, new.name);
            let new_sum: f64 = new.values.iter().sum();
            let old_sum: f64 = old.values.iter().sum();
            let val = (new.values[0] - old.values[0]) * 100.0 / (new_sum - old_sum);
            match elemnts_iter.peek() {
                Some(_) => print!("{:.1}, ", val),
                None => print!("{:.1}", val),
            };
        }
    }

    fn swap(&mut self) {
        std::mem::swap(&mut self.old_cpus, &mut self.new_cpus);
    }

    pub fn update_and_print(&mut self) {
        self.get_new();
        self.print_delta();
        self.swap();
    }

    pub fn print_header(&self) {
        let mut iter = self.old_cpus.iter().peekable();
        while let Some(cpu) = iter.next() {
            match iter.peek() {
                Some(_) => print!("{}, ", cpu.name),
                None => print!("{}", cpu.name),
            }
        }
        println!();
    }
}
