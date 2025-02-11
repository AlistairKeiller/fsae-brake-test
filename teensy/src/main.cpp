#include <Arduino.h>

void setup() {
  Serial.begin(9600);
  pinMode(A0, INPUT);
  pinMode(A1, INPUT);
}

void loop() {
  float val0 = analogRead(A0) * (3.3 / 1023.0);
  float val1 = analogRead(A1) * (3.3 / 1023.0);
  Serial.print(val0);
  Serial.print(',');
  Serial.println(val1);
  Serial.print('\n');
  delay(10);
}