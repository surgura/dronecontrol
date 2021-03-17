# Drone control
Simulation of and control algorithm for quad rotor drone.

## Mathematical model
Rotation of drone in axis-angle representation.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\theta}=\theta%20\underrightarrow{e},\lvert%20e\rvert=1)

Second derivative of rotation assuming a single motor. Arm ![](https://render.githubusercontent.com/render/math?math=\underrightarrow{r_F}) originates from drone center of gravity.

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\ddot{\underrightarrow{\theta}}=I^{-1}(\underrightarrow{r_F}%20\times%20\underrightarrow{F}))

Acceleration of drone from a single motor. Motor force is rotated by ![](https://render.githubusercontent.com/render/math?math=\theta).

![](https://render.githubusercontent.com/render/math?math=\color{%23666}%20\large%20\underrightarrow{\alpha}=\frac{1}{m}\underrightarrow{F^'}=\frac{1}{m}(\cos(\theta)\underrightarrow{F}%2B\sin(\theta)(\underrightarrow{e}\times%20\underrightarrow{F})%2B\underrightarrow{e}(1-\cos(\theta))(\underrightarrow{e}\cdot%20\underrightarrow{F})))

Position of drone.

![](https://render.githubusercontent.com/render/math?math=\ddot{\underrightarrow{r}}=\underrightarrow{\alpha})