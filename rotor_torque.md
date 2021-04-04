# Rotor torque
*See bottom of page for TLDR at conclusion.*

A quadcopter can change its yaw by having a different rotational speed between its clockwise and counterclockwise spinning rotors. Each rotor pushes against the air, causing both lift and a torque around the yaw axis. Increasing and decreasing the speed of rotors also changes the drone's rotation, just like a reaction wheel would, but this is generally negligible compared the aforementioned force. The torque is caused by the horizontal part of the interaction with air.

![](yaw.png)

Looking at the image above it should be clear that a rotor applies this force evenly in every direction. Note however the equation for torque:

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\tau}=\underrightarrow{r}\times%20\underrightarrow{F})

Where ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20%20\underrightarrow{r}) is the vector from the center of gravity of the drone to the point where the force where ![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20%20\underrightarrow{F}) is applied. The force applied further from the center of gravity thus has a larger influence on torque, and a rotor rotating at a constant speed will cause a rotational acceleration of the drone.

Questions arrise: How much yaw torque? And: How does this influence pitch and roll if the rotors are placed higher than the center of gravity?

## Math
*Keep in mind this is simplification meant to provide insight. Reality is a lot more complex. This is just a nice simplification.*

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20F): force applied by the rotor.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20r): distance from the center of the rotor to where the force is applied.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\underrightarrow{a}): arm from the center of gravity of the drone to the center of the rotor.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large\underrightarrow{\tau}=\displaystyle\int_0^{2\pi}(\underrightarrow{a}%2BR(\theta)\begin{pmatrix}r%5C%5C0%5C%5C0\end{pmatrix})\times)![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large(R(\theta)\begin{pmatrix}0%5C%5C0%5C%5CF\end{pmatrix})\delta\theta)

Where R is the rotation matrix of the rotor.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20%20R(\theta)=\begin{pmatrix}\cos(\theta)%260%26\sin{\theta}%5C%5C0%261%260%5C%5C-\sin(\theta)%260%26\cos(\theta)\end{pmatrix})

Solving

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large\underrightarrow{\tau}=\displaystyle\int_0^{2\pi}\begin{pmatrix}\underrightarrow{a}_x%2Br\cos(\theta)%5C%5C\underrightarrow{a}_y%5C%5C\underrightarrow{a}_z-r\sin(\theta)\end{pmatrix}\times)![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large\begin{pmatrix}F\sin(\theta)%5C%5C0%5C%5CF\cos(\theta)\end{pmatrix}\delta\theta)

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large\underrightarrow{\tau}=\displaystyle\int_0^{2\pi}\begin{pmatrix}F\underrightarrow{a}_y\cos(\theta)%5C%5CF(\underrightarrow{a}_z-r\sin(\theta))\sin(\theta)-F(\underrightarrow{a}_x%2Br\cos(\theta))\cos(\theta)%5C%5C-F\underrightarrow{a}_y\sin(\theta)\end{pmatrix}\delta\theta)

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large\underrightarrow{\tau}=F\displaystyle\int_0^{2\pi}\begin{pmatrix}\underrightarrow{a}_y\cos(\theta)%5C%5C\underrightarrow{a}_z\sin(\theta)-\underrightarrow{a}_x\cos(\theta)-r%5C%5C-\underrightarrow{a}_y\sin(\theta)\end{pmatrix}\delta\theta)

![](https://render.githubusercontent.com/render/math?math=\color{%23666}\large\underrightarrow{\tau}=\begin{pmatrix}0%5C%5C-2\pi%20rF%5C%5C0\end{pmatrix}\delta\theta)

That is quite nice. Only yaw is affected by this effect, even if the rotor is positioned higher or lower than the drone's center of mass. Keep in mind that the force ![](https://render.githubusercontent.com/render/math?math=\color{%23666}\underrightarrow{\F}) is a simplification of reality; there actually is force applied along the arm of the rotor. However, this would be modelled as an integral with varying ![](https://render.githubusercontent.com/render/math?math=\color{%23666}r) and ![](https://render.githubusercontent.com/render/math?math=\color{%23666}F), and as such has no influence on the above conclusions.

## Conclusion
Each rotor applies a downward facing force(lift) and torque around the yaw axis of the drone, both increasing with increasing rotational velocity.