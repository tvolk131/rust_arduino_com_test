#include <Stepper.h>

const int stepsPerRevolution = 200;
const int motorSpeed = 60;

String command;
int ping_interval_counter;

// TODO - See if we can convert these into a global array.
Stepper stepper0 = Stepper(stepsPerRevolution, 0, 1, 2, 3);
Stepper stepper1 = Stepper(stepsPerRevolution, 4, 5, 6, 7);
Stepper stepper2 = Stepper(stepsPerRevolution, 8, 9, 10, 11);
Stepper stepper3 = Stepper(stepsPerRevolution, 12, 13, 14, 15);
Stepper stepper4 = Stepper(stepsPerRevolution, 16, 17, 18, 19);
Stepper stepper5 = Stepper(stepsPerRevolution, 20, 21, 22, 23);
Stepper stepper6 = Stepper(stepsPerRevolution, 24, 25, 26, 27);
Stepper stepper7 = Stepper(stepsPerRevolution, 28, 29, 30, 31);
Stepper stepper8 = Stepper(stepsPerRevolution, 32, 33, 34, 35);
Stepper stepper9 = Stepper(stepsPerRevolution, 36, 37, 38, 39);
Stepper stepper10 = Stepper(stepsPerRevolution, 40, 41, 42, 43);
Stepper stepper11 = Stepper(stepsPerRevolution, 44, 45, 46, 47);
Stepper stepper12 = Stepper(stepsPerRevolution, 48, 49, 50, 51);

void setup() {
  Serial.begin(57600);
  Serial.setTimeout(500);

  ping_interval_counter = 0;

  stepper0.setSpeed(motorSpeed);
  stepper1.setSpeed(motorSpeed);
  stepper2.setSpeed(motorSpeed);
  stepper3.setSpeed(motorSpeed);
  stepper4.setSpeed(motorSpeed);
  stepper5.setSpeed(motorSpeed);
  stepper6.setSpeed(motorSpeed);
  stepper7.setSpeed(motorSpeed);
  stepper8.setSpeed(motorSpeed);
  stepper9.setSpeed(motorSpeed);
  stepper10.setSpeed(motorSpeed);
  stepper11.setSpeed(motorSpeed);
  stepper12.setSpeed(motorSpeed);
}

void loop() {
  if (Serial.available()) {
    command = Serial.readStringUntil('\n');
    command.trim();
    if (command.equals("list_commands")) {
      Serial.println("Success: stepper0, stepper1, stepper2, stepper3, stepper4, stepper5, stepper6, stepper7, stepper8, stepper9, stepper10, stepper11, stepper12");
    } else if (command.equals("stepper0")) {
      stepper0.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper1")) {
      stepper1.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper2")) {
      stepper2.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper3")) {
      stepper3.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper4")) {
      stepper4.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper5")) {
      stepper5.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper6")) {
      stepper6.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper7")) {
      stepper7.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper8")) {
      stepper8.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper9")) {
      stepper9.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper10")) {
      stepper10.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper11")) {
      stepper11.step(stepsPerRevolution);
      Serial.println("Success");
    } else if (command.equals("stepper12")) {
      stepper12.step(stepsPerRevolution);
      Serial.println("Success");
    } else {
      Serial.println("Error: Bad Command");
    }
  }

  if (ping_interval_counter == 10000) {
    ping_interval_counter = 0;
    Serial.println("Ping");
  }
  ping_interval_counter++;
}