# a REPL for hecksagon

import subprocess
# Im too lazy to make it in rust

def main():
    with open("tstd.h2", "w") as f:
        while True:
            input = input("> ") 
            if input == "..": subprocess.call("hecksagon tstd.h2", shell=True); break 
            f.write(input)
            f.write("\n")

if __name__ == "__main__":
    main()
else:
    print("this file is not meant to be imported. if you are trying to extend the REPL please do a PR please. ")
