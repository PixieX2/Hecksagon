import subprocess, os
# This script will build the interpreter and copy it to /usr/opt/hecksagon
cmd = "make update" # this will build AND install 
subprocess.call(cmd, shell=True)
os.remove("*.pdb") # remove pdb files (they're useless unless you're debugging)

print("Done. Now you can run hecksagon. It's in /usr/opt/hecksagon")