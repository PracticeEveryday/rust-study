#!/bin/bash
yum update -y
curl https://sh.rustup.rs -sSf > RUSTUP.sh
sh RUSTUP.sh -y
rm RUSTUP.sh
yum install gcc -y
echo "Installing for ec2-user.."
cp -r ~/.{cargo,rustup,bash_profile,profile} /home/ec2-user/
echo "ec2-user ready."