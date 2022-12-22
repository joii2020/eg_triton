# Notes about triton-vm

* Triton-vm 在执行之前必须运行 ```cargo run --bin constraint-evaluation-generator```
* triton-vm 在 verify 中，需要： input\output，不需要program。
* verify 的耗时长于程序正常的执行， proof时间更长


## 疑问
* 为什么在Stark的初始化的时候，需要用到process，实际的代码里并没有用到
* 关于input与output参与prove运算的问题

## constraint-evaluation-generator
会生成 ***table_constraints.rs
* TODO  需要进一步确认这些数据的来源

## simulate
正常的虚拟机，这里使用的指令集为： https://triton-vm.org/spec/arithmetization.html

## prove


## verify
