set print pretty on
target remote :3333
monitor arm semihosting enable
load
break main
continue
