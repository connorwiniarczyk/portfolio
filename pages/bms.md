# Battery Management System

## Designed for the Lafayette 2020 FSAE Car Team

---

![](/media/fsae/car_drawing.jpg)

Each year, members of Lafayette's Electrical and Mechanical Engineering
departments work to build a fully functioning electric car in order to
compete in the FSAE formula hybrid competition. The vast majority of the car
is designed and manufactured in house by students.


Unlike a number of other FSAE teams, Lafayette's is taught as a senior design class, and has a yearly turnover rate of approximately 100%, meaning the onset of COVID-19 in the spring of 2020 cut our team's time on the project tragically short. While we were unable to meet our goal, I'm hoping that some of our work can be preserved here. 

The Car is named "Sparky" in honor of the many, many sparks that we made while creating it. 

For a more complete overview of the project, check out these links:

- Project Website: [http://sites.lafayette.edu/motorsports](http://sites.lafayette.edu/motorsports)
- Official Competition Website: [https://formula-hybrid.org/](https://formula-hybrid.org/)
- Official Project GitHub page: [https://github.com/orgs/Lafayette-FSAE](https://github.com/orgs/Lafayette-FSAE)

## The Battery Packs

![pack drawing](/media/bms/pack_drawing.PNG)

The battery packs deliver high voltage power to the vehicle's Tractive System. Each of the two packs supply about 54V through 16 prismatic LiFePO4 cells, each of which is monitored through a Battery Management System (BMS).

The BMS consists of a Pack Manager (PacMan) and 16 Cell Manager (CellMan) boards, one per cell. The two packs are wired in series to provide ~108 V to the assembled accumulator system. The packs interacts with other car subsystems via a CAN bus and a relay based safety system.


### Teammates

- Jon Abel
- Tim LaGreca
- Clement Hathaway
- Simone Khalifa
- Dwayne Whittaker

### Features and Specs

- Chemistry: Lithium Iron Phosphate (LiFePO4) 
- Nominal Pack Voltage: 51.2 V
- Maximum Current: 300A
- Maximum Operating Temperature: 65C
- CAN interface fully compliant with CANOpen standard
- Fully featured display / control panel for field configuration and problem diagnosing 
- Real time voltage and temperature measurements for all Cells
- Support for active cell balancing 

![pack connectivity](/media/bms/pack_connectivity.jpg)

### Construction

### CellMan and PacMan
