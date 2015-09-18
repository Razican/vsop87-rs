#!/usr/bin/env python3
import os

with open("data/vsop87.chk") as data:
    outfile = open("tests/vsop87_tests.rs", 'w')
    outfile.write("extern crate vsop87;\nuse vsop87::vsop87::*;\n");

    current_planet = ""
    jde = ""
    new_planet = True
    for line in data:
        if line == "\n":
            line = next(data)

        line = line.split()
        if line[0] == "VSOP87":
            jde = line[2][2:]

            if line[1].lower().replace("-", "_") != current_planet:
                if current_planet != "":
                    outfile.write("}\n")
                current_planet = line[1].lower().replace("-", "_")
                outfile.write("\n#[test]\nfn it_"+ current_planet +"() {\n")
                new_planet = True

        elif not line[0].startswith("VSOP87") and line[0] != "\x1a":
            if not new_planet:
                outfile.write("\n")
            else:
                new_planet = False

            outfile.write("    let (a, l, k, h, q, p) = "+ current_planet +"("+ jde +");\n\n")

            try:
                second_line = next(data).split()
            except:
                print("No more lines. Last line:")
                print(line)

            outfile.write("    assert!(a > {0:.10f} && a < {1:.10f});\n"
                .format(float(line[1])-0.0000000001, float(line[1])+0.0000000001))
            outfile.write("    assert!(l > {0:.10f} && l < {1:.10f});\n"
                .format(float(second_line[1])-0.0000000001, float(second_line[1])+0.0000000001))

            outfile.write("    assert!(k > {0:.10f} && k < {1:.10f});\n"
                .format(float(line[4])-0.0000000001, float(line[4])+0.0000000001))
            outfile.write("    assert!(h > {0:.10f} && h < {1:.10f});\n"
                .format(float(second_line[4])-0.0000000001, float(second_line[4])+0.0000000001))

            outfile.write("    assert!(q > {0:.10f} && q < {1:.10f});\n"
                .format(float(line[7])-0.0000000001, float(line[7])+0.0000000001))
            outfile.write("    assert!(p > {0:.8f} && p < {1:.8f});\n"
                .format(float(second_line[7])-0.00000038, float(second_line[7])+0.00000038))

        elif line[0] == "VSOP87A":
            print("Found VSOP87A")
            outfile.write("}\n")
            outfile.close()
            print("Finished VSOP87")
            break
