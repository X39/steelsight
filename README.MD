# Build Yourself

1. Install rust
2. Install OpenCV (https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md)
    1. Windows
        1. Install vcpkg https://learn.microsoft.com/de-de/vcpkg/get_started/get-started?pivots=shell-powershell
            1. Create a new, empty folder anywhere on your system
            2. Inside that new folder, in Windows Explorer, hold `SHIFT` and click the `Right Mouse Button`
            3. Click on the `Terminal` or `Console` entry (depending on Windows Version and Language, the naming might
               be different)
            4. Run `git clone https://github.com/microsoft/vcpkg.git .`
            5. Run `.bootstrap-vcpkg.bat`
            6. Run `SystemPropertiesAdvanced.exe`
            7. In the newly created window, click on the last button displayed (`Environment variables`)
            8. In the newly created window, Select the `Path` variable of either your user (top) or your system (bottom)
            9. Click `Modify`
            10. Copy the path of your vcpkg installation by right clicking the path
            11. In the modify environment variables window, click new and paste (`CTRL + V`) the path in
            12. Click OK
            13. Click OK
            14. Click OK
            15. Close any windows you have opened during this process
        2. Install OpenCV using VCPKG
            1. Install Visual Studio Community https://visualstudio.microsoft.com/downloads/
            2. Install the default setup
            3. Close any windows that may have opened
            4. Using the start menu, open the `Visual Studio Installer`
            5. Click the `Change` button (top one, out of the four `Change` `Start` `More` and `Update`)
            6. Select the second tab (`Individual Components`)
            7. Next one is a little bit tricky:
                1. In the search column, search for `MSVC` and find the ticket entry (Should look like
                   `MSVC v143 - VS 2022 C++-x64/x86...`)
                2. Write down or remember that `v143` part (you might have a newer version)
                3. Now search for `ATL v143` (replace the `v143` part with your version number)
                4. Go through the list and search for the entry that does not have a leading version (your `v143` will
                   be in all of them, but there are different version numbers too), no `Spectre` inside of it and `x86`
                   and `x64` in it (one of the ones at the very bottom, most likely, on the list) and tick it
               5. Confirm the change using the `Change` button at the very bottom right corner of that window.
               6. Confirm the admin prompt
               7. Wait until everything is installed.
            8. Open the terminal by right clicking the windows logo in your task bar and selecting `Terminal` or
               `Console` (depending on Windows Version and Language, the naming might be different)
            9. Run `vcpkg install llvm opencv4[contrib,nonfree]:x64-windows-static-md`
        3. Add LLVM to `PATH` variable
           1. See "Install vcpkg" for modifying environment variables
           2. In your vcpkg installation, find the path to `<your_vcpkg_path>/packages/llvm_x64-windows/`
           3. Copy that path
           4. Create an entry for `<your_vcpkg_path>/packages/llvm_x64-windows/tools/llvm/`
           5. Create an entry for `<your_vcpkg_path>/packages/llvm_x64-windows/bin/`
3. Run `cargo build --release`