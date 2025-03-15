#!/bin/bash

echo "deb [trusted=yes] https://MostlyKIGuess.github.io/Yappus-Term/apt stable main" | sudo tee /etc/apt/sources.list.d/yappus.list
sudo apt update
sudo apt install yappus
