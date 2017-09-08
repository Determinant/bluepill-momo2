bluepill-momo
==============
A minimal example using USB CDC.

Try out
=======

Build by:

::

    xargo build --release

Extract the image:

::

    arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/bluepill-momo bp

Flash the image and test run (suppose the device is discovered as ``/dev/hidraw3``):

::

    sudo -i
    # in root:
    cat /dev/hidraw3
    # at the same time try to write to the device to send HID reports
    printf "\0heeeeeeeeeeeeeeeeeeeeeelo\n\0" > /dev/hidraw3
    printf "\0wooooooooooooooooooooorld\n\0" > /dev/hidraw3
    printf "\0ooooooooooooooooooooooops\n\0" > /dev/hidraw3
