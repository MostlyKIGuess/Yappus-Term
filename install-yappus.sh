#!/bin/bash


echo "Installing Yappus Terminal v1.1.0..."

# Download the latest .deb package
echo "Downloading package..."
curl -L -o /tmp/yappus-term_1.1.0-1_amd64.deb https://github.com/MostlyKIGuess/Yappus-Term/releases/download/v1.1.0/yappus-term_1.1.0-1_amd64.deb

if [ $? -ne 0 ]; then
    echo "Failed to download Yappus Terminal package"
    exit 1
fi

# Install the package
echo "Installing Yappus Terminal..."
sudo dpkg -i /tmp/yappus-term_1.1.0-1_amd64.deb

echo "Installing dependencies..."
sudo apt-get install -f -y

echo "Cleaning up..."
rm /tmp/yappus-term_1.1.0-1_amd64.deb

echo "Yappus Terminal installed successfully!"
echo "Run 'yappus' to start using the application."
echo ""
echo "For more information, visit: https://github.com/MostlyKIGuess/Yappus-Term"
