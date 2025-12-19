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
    print("NOW outdated, use hecksagon --repl or hecksagon --stdin instead.")
    main()
else:
    print("this file is not meant to be imported.  ̶i̶f̶ ̶y̶o̶u̶ ̶a̶r̶e̶ ̶t̶r̶y̶i̶n̶g̶ ̶t̶o̶ ̶e̶x̶t̶e̶n̶d̶ ̶t̶h̶e̶ ̶R̶E̶P̶L̶ ̶p̶l̶e̶a̶s̶e̶ ̶d̶o̶ ̶a̶ ̶P̶R̶ ̶p̶l̶e̶a̶s̶e̶.̶   Nevermind dont do PR for this, i will close the PR ")
# NOW outdated, use hecksagon --repl or hecksagon --stdin
