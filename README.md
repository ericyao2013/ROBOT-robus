[![Build Status](https://travis-ci.com/pollen/robus.svg?token=RxFY2cvxnBdyPk3Jevf4&branch=master)](https://travis-ci.com/pollen/robus)

# Robus: the Robust Robotic Bus

Robus lets you seamlessly connect robotic actuators and sensors using a unified bus. New modules are automatically detected thanks to the topology detection algorithm.

Each robus node is called a module and represent a function in your robot - typically a sensor or actuator. Modules can directly communicate with other modules using broadcast, specific module ids or specific module groups.

Robus is light-weighted and designed for the embedded world.

Robus reduces the time between an idea to the prototype. It provides a unified messaging architecture for modular robotics, all modules can be connected on the same 5 wires bus containing both power and 2 communication bus.
