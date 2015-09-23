#!/usr/bin/env python3
import os

for filename in os.listdir("data"):
    if "sun" in filename:
        newfile = "sun.rs"
    if "mer" in filename:
        newfile = "mercury.rs"
    elif "ven" in filename:
        newfile = "venus.rs"
    elif "emb" in filename:
        newfile = "earth_moon.rs"
    elif "ear" in filename:
        newfile = "earth.rs"
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
        with open("data/"+filename) as data:
            outfile = open("src/"+newfile, 'w')
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
                    elif "VARIABLE 3" in line:
                        if (letter != "K"):
                            letter = "K"
                            number = 0
                    elif "VARIABLE 4" in line:
                        if (letter != "H"):
                            letter = "H"
                            number = 0
                    elif "VARIABLE 5" in line:
                        if (letter != "Q"):
                            letter = "Q"
                            number = 0
                    elif "VARIABLE 6" in line:
                        if (letter != "P"):
                            letter = "P"
                            number = 0
            outfile.close()

    elif (filename.startswith("VSOP87A.")):
        with open("data/"+filename) as data:
            outfile = open("src/vsop87a/"+newfile, 'w')
            number = 0
            letter = "X"
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
                    if letter != "X" or number is not 0:
                        outfile.write("\n")
                    outfile.write("pub const "+ letter + str(number) +": [(f64, f64, f64); "+ str(size) +"] = [\n")
                    outfile.write(current_var)
                    outfile.write("];\n")

                    number += 1
                    size = 0
                    current_var = ""

                    if "VARIABLE 2" in line:
                        if (letter != "Y"):
                            letter = "Y"
                            number = 0
                    elif "VARIABLE 3" in line:
                        if (letter != "Z"):
                            letter = "Z"
                            number = 0
            outfile.close()

    elif (filename.startswith("VSOP87B.")):
        with open("data/"+filename) as data:
            outfile = open("src/vsop87b/"+newfile, 'w')
            number = 0
            letter = "L"
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
                    if letter != "L" or number is not 0:
                        outfile.write("\n")
                    outfile.write("pub const "+ letter + str(number) +": [(f64, f64, f64); "+ str(size) +"] = [\n")
                    outfile.write(current_var)
                    outfile.write("];\n")

                    number += 1
                    size = 0
                    current_var = ""

                    if "VARIABLE 2" in line:
                        if (letter != "B"):
                            letter = "B"
                            number = 0
                    elif "VARIABLE 3" in line:
                        if (letter != "R"):
                            letter = "R"
                            number = 0
            outfile.close()

    elif (filename.startswith("VSOP87C.")):
        with open("data/"+filename) as data:
            outfile = open("src/vsop87c/"+newfile, 'w')
            number = 0
            letter = "X"
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
                    if letter != "X" or number is not 0:
                        outfile.write("\n")
                    outfile.write("pub const "+ letter + str(number) +": [(f64, f64, f64); "+ str(size) +"] = [\n")
                    outfile.write(current_var)
                    outfile.write("];\n")

                    number += 1
                    size = 0
                    current_var = ""

                    if "VARIABLE 2" in line:
                        if (letter != "Y"):
                            letter = "Y"
                            number = 0
                    elif "VARIABLE 3" in line:
                        if (letter != "Z"):
                            letter = "Z"
                            number = 0

            outfile.close()
