# QuantumLT cli

Quantum LT 16をコントロールする

- init

## init

- In 1-16 => Mute
- DAW 1/2 => AUX 1/2
- DAW 3/4 => AUX 2/4
- DAW 5/6 => AUX 5/6
- DAW 7/8 => AUX 7/8
- DAW 1-10 => Loop 9/10

## BUILD

```sh
# WSLでaarch64向けビルド
cargo build --target aarch64-unknown-linux-gnu
```

## USB接続時に実行

```sh
# /opt/quantumLT-cli/quantumLT-cli
# /opt/quantumLT-cli/systemd/quantumLT-init.service
# /opt/quantumLT-cli/udev/quantumLT.rules

sudo ln -s /opt/quantumLT-cli/udev/quantumLT.rules /etc/udev/rules.d/99-quantumLT.rules
sudo ln -s /opt/quantumLT-cli/systemd/quantumLT-init.service /etc/systemd/system/quantumLT-init@.service
```
