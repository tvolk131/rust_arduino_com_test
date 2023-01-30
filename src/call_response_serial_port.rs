use serialport::SerialPort;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::time::Duration;

#[derive(serde::Deserialize, Debug)]
pub struct ArduinoCommandResponse {
    status: String,
    command: Option<String>,
    response: Option<serde_json::Value>,
}

#[derive(std::fmt::Debug)]
pub enum SerialError {
    Timeout,
    MalformedResponse
}

pub struct CallResponseSerialPort {
    port: Box<dyn SerialPort>,
    supported_commands: HashSet<String>,
    timeout_to_retry: Duration,
    max_retries: u32,
}

impl CallResponseSerialPort {
    pub fn new(port: Box<dyn SerialPort>) -> Result<Self, SerialError> {
        let mut p = Self {
            port,
            supported_commands: HashSet::new(),
            timeout_to_retry: Duration::from_millis(20000),
            max_retries: 10,
        };

        for command in p.get_commands()? {
            p.supported_commands.insert(command);
        }

        Ok(p)
    }

    fn get_commands(&mut self) -> Result<Vec<String>, SerialError> {
        let response = match self.execute_command("list_commands") {
            Ok(response) => response,
            Err(err) => return Err(err)
        };

        let raw_command_values = match &response.response {
            Some(res) => {
                match res.as_array() {
                    Some(commands) => commands,
                    None => return Err(SerialError::MalformedResponse)
                }
            },
            None => return Err(SerialError::MalformedResponse)
        };

        let mut command_strings = Vec::new();
        for raw_command_value in raw_command_values {
            match raw_command_value.as_str() {
                Some(command_string) => command_strings.push(command_string.to_string()),
                None => return Err(SerialError::MalformedResponse)
            };
        }

        Ok(command_strings)
    }

    pub fn get_supported_commands(&self) -> &HashSet<String> {
        &self.supported_commands
    }

    pub fn execute_command(&mut self, command: &str) -> Result<ArduinoCommandResponse, SerialError> {
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
    ) -> Result<ArduinoCommandResponse, SerialError> {
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
        self.wait_for_input()
    }

    fn wait_for_input(&mut self) -> Result<ArduinoCommandResponse, SerialError> {
        let start_time = std::time::Instant::now();
        let mut buffer = [0; 10000];
        let mut stringified_buffer = String::new();
        loop {
            let num_bytes_available = self.port.bytes_to_read().unwrap();
            if num_bytes_available > 0 {
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
                    return match serde_json::from_str::<ArduinoCommandResponse>(line) {
                        Ok(response) => Ok(response),
                        Err(_) => Err(SerialError::MalformedResponse)
                    };
                }
            }
            if std::time::Instant::now().duration_since(start_time) > self.timeout_to_retry {
                return Err(SerialError::Timeout);
            }
            self.port.clear(serialport::ClearBuffer::Output).unwrap();
        }
    }
}
