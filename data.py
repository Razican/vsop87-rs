#!/usr/bin/env python3
import os

for filename in os.listdir("/home/razican/Downloads/vsop/"):
    if "mer" in filename:
        newfile = "mercury.rs"
    elif "ven" in filename:
        newfile = "venus.rs"
    elif "emb" in filename:
        newfile = "earth_moon.rs"
    elif "mar" in filename:
        newfile = "mars.rs"
    elif "jup" in filename:
        newfile = "jupiter.rs"
    elif "sat" in filename:
        newfile = "saturn.rs"
    elif "ura" in filename:
        newfile = "uranus.rs"
    elif "nep" in filename:
        newfile = "neptune.rs"

    if (filename.startswith("VSOP87.")):
        print("Opening the file "+filename)

        with open("/home/razican/Downloads/vsop/"+filename) as data:

            outfile = open("src/vsop87/"+newfile, 'w')
            number = 0
            letter = "A"
            size = 0
            current_var = ""
            next(data);
            for line in data:
                line = line.replace("-", " -")
                numbers = line.split()

                if numbers[0] != "VSOP87":
                    current_var += "    ("+ numbers[16] +", "+ numbers[17] +", "+ numbers[18] +"),\n"
                    size += 1
                else:
                    if letter != "A" or number is not 0:
                        outfile.write("\n")
                    outfile.write("pub const "+ letter + str(number) +": [(f64, f64, f64); "+ str(size) +"] = [\n")
                    outfile.write(current_var)
                    outfile.write("];\n")

                    number += 1
                    size = 0
                    current_var = ""

                    if "VARIABLE 2" in line:
                        if (letter != "L"):
                            letter = "L"
                            number = 0
                            print("First variable finished")
                    elif "VARIABLE 3" in line:
                        if (letter != "K"):
                            letter = "K"
                            number = 0
                            print("Second variable finished")
                    elif "VARIABLE 4" in line:
                        if (letter != "H"):
                            letter = "H"
                            number = 0
                            print("Third variable finished")
                    elif "VARIABLE 5" in line:
                        if (letter != "Q"):
                            letter = "Q"
                            number = 0
                            print("Fourth variable finished")
                    elif "VARIABLE 6" in line:
                        if (letter != "P"):
                            letter = "P"
                            number = 0
                            print("Fifth variable finished")

            print("Sixth variable finished")
            outfile.close()
