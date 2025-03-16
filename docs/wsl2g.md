Contains info on a Wayland permissions bug and missing symlinks in current wsl2 as of the time of writing. 
https://github.com/microsoft/WSL/issues/11542	

https://github.com/microsoft/wslg/issues/1032#issuecomment-2345292609

# WSL2 Audio Issues

To fix audio issues in WSL2:

1. Follow the solution described in this GitHub comment: https://github.com/microsoft/WSL/issues/2187#issuecomment-2605861048

2. Add the following line to your `.bashrc` file:
   ```bash
   export PULSE_SERVER=unix:/mnt/wslg/PulseServer
   ```

3. Source your `.bashrc` file or restart your WSL2 terminal:
   ```bash
   source ~/.bashrc
   ```