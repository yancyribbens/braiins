####################################################################################################
# Copyright (C) 2019  Braiins Systems s.r.o.
#
# This file is part of Braiins Open-Source Initiative (BOSI).
#
# BOSI is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#
# Please, keep in mind that we may also license BOSI or any part thereof
# under a proprietary license. For more information on the terms and conditions
# of such proprietary license or if you have any other questions, please
# contact us at opensource@braiins.com.
####################################################################################################

####################################################################################################
# Connection to FPGA, loading bitstream and initialization
####################################################################################################
source test_init.tcl

####################################################################################################
# Test of GPIO LEDs
####################################################################################################
# LEDs are on GPIO_1, pin 2 - PL_LED1 (green on panel)
mwr 0x41210000 4
puts "Set PL_LED1 on"
exec sleep 5
# LEDs are on GPIO_1, pin 3 - PL_LED2 (red on panel)
mwr 0x41210000 8
puts "Set PL_LED2 on"
exec sleep 5
# LEDs are on GPIO_1, pin 4 - PL_LED3 (red on board)
mwr 0x41210000 16
puts "Set PL_LED3 on"
exec sleep 5
# turn off all LEDs
mwr 0x41210000 0
puts "Set all LEDs off"

####################################################################################################
# Test of VID generator
####################################################################################################
puts "Test of VID generator"
# send value of 0x23 and enable generation (mask 0x100)
mwr 0x43C50000 0x123
# wait some time or check if all data are sent
exec sleep 5
# disable generation
mwr 0x43C50000 0x0

####################################################################################################
# Test of FANs
####################################################################################################
source test_fan.tcl

# Timer 0 - FAN1 and FAN2
test_fan "FAN 1&2" $FAN_A
# Timer 1 - FAN3 and FAN4
test_fan "FAN 3&4" $FAN_B
# Timer 2 - FAN5 and FAN6
test_fan "FAN 5&6" $FAN_C

####################################################################################################
# Test of SPI modules
####################################################################################################
source test_spi.tcl

# SPI module 0
test_spi "SPI 0, high speed" 0x41E00000 0x12
# SPI module 1
test_spi "SPI 1, high speed" 0x41E10000 0x34
# SPI module 2
test_spi "SPI 2, high speed" 0x41E20000 0x56
# SPI module 3
test_spi "SPI 3, high speed" 0x41E30000 0x78
exec sleep 5

####################################################################################################
# Test of change FCLK1 frequency
####################################################################################################
# unlock access to  System Level Control Registers (SCLR)
mwr 0xF8000008 0xDF0D
# change frequency of FCLK1 to 1/4 (original value is 0x00400500)
mwr 0xF8000180 0x01000500

# SPI module 0
test_spi "SPI 0, low speed" 0x41E00000 0x12
# SPI module 1
test_spi "SPI 1, low speed" 0x41E10000 0x34
# SPI module 2
test_spi "SPI 2, low speed" 0x41E20000 0x56
# SPI module 3
test_spi "SPI 3, low speed" 0x41E30000 0x78

# change frequency of FCLK1 back to 3.125 MHz
mwr 0xF8000180 0x00400500
# lock access to  System Level Control Registers (SCLR)
mwr 0xF8000004 0x767B
