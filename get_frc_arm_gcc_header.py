import subprocess as sp
import re
import os

def main():
    regex = '(?:#include ["<]\.\.\.[">] search starts here:\n){2}((?:(?:.*?)\n)*)(?:End of search list\.)'

    compiler = os.getenv("FRC_GXX")

    p = sp.Popen([compiler, '-E', '-W', '-v', '-'], stdin=sp.PIPE, stdout=sp.PIPE, stderr=sp.PIPE, shell=False)

    stdout, stderr = p.communicate(b'\n')

    rc = p.returncode

    header_dirs = re.search(regex, stderr).group(1).strip().split('\n')

    for x in header_dirs:
        x = x.strip()

        print(x)

if __name__ == "__main__":
    main()