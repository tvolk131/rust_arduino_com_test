use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;

#[derive(std::fmt::Debug)]
pub enum SerialError {
    Timeout,
}

pub struct CallResponseSerialPort {
    port: Box<dyn SerialPort>,
    timeout_to_retry: Duration,
    max_retries: u32,
}

impl CallResponseSerialPort {
    pub fn new(port: Box<dyn SerialPort>) -> Result<Self, SerialError> {
        let mut p = Self {
            port,
            timeout_to_retry: Duration::from_millis(20000),
            max_retries: 10,
        };
        p.wait_for_input(false)?;
        Ok(p)
    }

    pub fn get_commands(&mut self) -> Result<Vec<String>, SerialError> {
        return self.execute_command("list_commands").map(|response| {
            match response.split("Success: ").collect::<Vec<&str>>().get(1) {
                Some(split) => *split,
                None => &response,
            }
            .split(',')
            .map(|command| command.trim().to_string())
            .collect()
        });
    }

    pub fn execute_command(&mut self, command: &str) -> Result<String, SerialError> {
        for _ in 0..self.max_retries {
            if let Ok(response) = self.execute_command_give_up_after_timeout(command) {
                return Ok(response);
            }
        }
        return self.execute_command_give_up_after_timeout(command);
    }

    fn execute_command_give_up_after_timeout(
        &mut self,
        command: &str,
    ) -> Result<String, SerialError> {
        let mut buffer = [0; 10000];
        self.port.clear(serialport::ClearBuffer::All).unwrap();
        let num_bytes_available = self.port.bytes_to_read().unwrap();
        // Clear out any potential leftover bytes.
        if num_bytes_available > 0 {
            self.port
                .read(&mut buffer[..(num_bytes_available as usize)])
                .unwrap();
        }

        for char in format!("{}\n", command).chars() {
            self.port.write_all(format!("{}", char).as_bytes()).unwrap();
            self.port.flush().unwrap();
            std::thread::sleep(Duration::from_millis(1));
        }
        self.wait_for_input(true)
    }

    fn wait_for_input(&mut self, discard_pings: bool) -> Result<String, SerialError> {
        let start_time = std::time::Instant::now();
        let mut buffer = [0; 10000];
        let mut stringified_buffer = String::new();
        loop {
            let num_bytes_available = self.port.bytes_to_read().unwrap();
            if num_bytes_available > 0 {
                // println!("Bytes to read: {:?}", num_bytes_available);
                let read_result = self
                    .port
                    .read(&mut buffer[..(num_bytes_available as usize)]);
                match read_result {
                    Ok(bytes_read) => {
                        if let Ok(text) = String::from_utf8(Vec::from(&buffer[..bytes_read])) {
                            stringified_buffer += &text;
                        }
                    }
                    Err(err) => println!("Got error: {}", err),
                };
            }
            if stringified_buffer.ends_with("\n") {
                for line in stringified_buffer.split("\n").map(|line| line.trim()) {
                    if !line.is_empty() && !(line == "Ping" && discard_pings) {
                        return Ok(line.to_string());
                    }
                }
            }
            if std::time::Instant::now().duration_since(start_time) > self.timeout_to_retry {
                return Err(SerialError::Timeout);
            }
            self.port.clear(serialport::ClearBuffer::Output).unwrap();
        }
    }
}
