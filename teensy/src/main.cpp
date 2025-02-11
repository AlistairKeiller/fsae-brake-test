#include <Arduino.h>

void setup() {
  Serial.begin(9600);
  pinMode(A0, INPUT);
  pinMode(A1, INPUT);
}

void loop() {
  int raw0 = analogRead(A0);
  int raw1 = analogRead(A1);

  constexpr float CONVERSION = 3.3f / 1023.0f;

  float val0 = raw0 * CONVERSION;
  float val1 = raw1 * CONVERSION;

  Serial.printf("%.3f,%.3f\n", val0, val1);

  delay(10);
}