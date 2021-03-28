# Rotor torque
A quadcopter can change its yaw by having a different rotational speed between its clockwise and counterclockwise spinning rotors. Each rotor pushes against the air, causing a torque, mainly around the yaw axis. Increasing and decreasing the speed of rotors also changes the drone's rotation, just like a reaction wheel would, but this is generally negligible compared the aforementioned force.

![](yaw.png)

Looking at the image above it should be clear that a rotor applies a force evenly in every direction. Note however the equation for torque:

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\tau}=\underrightarrow{r}\times%20\underrightarrow{F})

Where ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20%20\underrightarrow{r}) is the vector from the center of gravity of the drone to the point where the force where ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20%20\underrightarrow{F}) is applied. The force applied further from the center of gravity thus has a larger unfluence on torque, and a rotor rotating at a constant speed will cause a rotational acceleration of the drone.

Questions arrise: How much yaw torque? And: How does this influence pitch and roll if the rotors are placed higher than the center of gravity?

## Math
![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20F): force applied by the rotor.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20r): distance from the center of the rotor to where the force is applied.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\underrightarrow{a}): arm from the center of gravity of the drone to the center of the rotor.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large\underrightarrow{\tau}=\displaystyle\int_0^{2\pi}(\underrightarrow{a}%2BR(\theta)\begin{pmatrix}r%5C%5C0%5C%5C0\end{pmatrix})\times(R(\theta)\begin{pmatrix}0%5C%5C0%5C%5CF\end{pmatrix})\delta\theta)

Where R is the rotation matrix of the rotor.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20%20R(\theta)=\begin{pmatrix}\cos(\theta)%260%26\sin{\theta}%5C%5C0%261%260%5C%5C-\sin(\theta)%260%26\cos(\theta)\end{pmatrix})