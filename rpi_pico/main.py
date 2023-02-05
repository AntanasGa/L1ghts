from machine import Pin, PWM, mem32
import utime
from time import sleep
from i2cSlave import i2c_slave

address = 0x08

# identifies device:
#  * 0x10 - light controller
#  * 0x20 - presence detector
peripheral_identifier = 0x10

responder = i2c_slave(0, sda=0, scl=1, slaveAddress=address)
counter = 0
indicator = Pin(25, Pin.OUT)
indicator.value(0)
leds = [
    PWM(Pin(3)),
    PWM(Pin(5)),
    PWM(Pin(7)),
    PWM(Pin(8)),
    PWM(Pin(9)),
    PWM(Pin(11)),
    PWM(Pin(13)),
    PWM(Pin(14)),
    PWM(Pin(15)),
    PWM(Pin(18)),
    PWM(Pin(20)),
    PWM(Pin(22)),
    PWM(Pin(26)),
    PWM(Pin(28)),
]
 
 # reference for faster data transfer
led_bit_value = []
 
 # initializing pwm
for i in leds:
    i.freq(1000)
    i.duty_u16(0)
    for _ in [0, 1]:
        # setting 2 bits per signal
        led_bit_value.append(0)

for e in range(10):
    indicator.toggle()
    sleep(0.05)

content = []
content_action = []

try:
    while True:
        while responder.any():
            content.append(responder.get())
        if (len(content) != 0):
            # end of transmition
            content_action = content.copy()
            content = []
            if (len(content_action) == 1):
                counter = content_action[0]
            if (len(content_action) == len(led_bit_value)):
                # on the right amount of bytes we update pwm
                led_bit_value = content_action.copy()
                local_counter = 0
                for i in leds:
                    intensity = 0
                    for o in [0, 1]:
                        led_bit_value[(local_counter * 2) + o] &= 0xff
                        if o == 0:
                            intensity = led_bit_value[local_counter * 2] * 256
                            continue
                        intensity = intensity + led_bit_value[(local_counter * 2) + 1]
                    local_counter += 1
                    i.duty_u16(intensity & 0xffff)

        if responder.anyRead():
            data_map = [peripheral_identifier, len(leds)]
            data_map.extend(led_bit_value)
            transmitted = 0x00
            if len(data_map) > counter and type(data_map[counter]) != None:
                transmitted = data_map[counter]
            responder.put(transmitted & 0xff)
            counter = counter + 1
            if (counter > 255):
                counter = 0
except KeyboardInterrupt:
    pass

