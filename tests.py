#!/usr/bin/env python3
import os

with open("data/vsop87.chk") as data:
    outfile = open("tests/vsop87_tests.rs", 'w')
    outfile.write("extern crate vsop87;\n");
    current = "vsop87"

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

        elif current == "vsop87" and not line[0].startswith("VSOP87") and line[0] != "\x1a":
            if not new_planet:
                outfile.write("\n")
            else:
                new_planet = False

            outfile.write("    let (a, l, k, h, q, p) = vsop87::"+ current_planet +"("+ jde +");\n\n")

            second_line = next(data).split()

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
            if current == "vsop87":
                current = "vsop87a"
                current_planet = ""
                print("Found VSOP87A")
                outfile.write("}\n")
                outfile.close()
                print("Finished VSOP87")
                outfile = open("tests/vsop87a_tests.rs", 'w')
                outfile.write("extern crate vsop87;\nuse vsop87::*;\n");

            jde = line[2][2:]
            if line[1].lower().replace("-", "_") != current_planet:
                if current_planet != "":
                    outfile.write("}\n")
                current_planet = line[1].lower().replace("-", "_")
                outfile.write("\n#[test]\nfn it_"+ current_planet +"() {\n")
                new_planet = True

        elif current == "vsop87a" and not line[0].startswith("VSOP87") and line[0] != "\x1a":
            if not new_planet:
                outfile.write("\n")
            else:
                new_planet = False

            outfile.write("    let (x, y, z) = vsop87a::"+ current_planet +"("+ jde +");\n\n")

            next(data)
            # try:
            #     second_line = next(data).split()
            # except:
            #     print("No more lines. Last line:")
            #     print(line)

            outfile.write("    assert!(x > {0:.10f} && x < {1:.10f});\n"
                .format(float(line[1])-0.0000000001, float(line[1])+0.0000000001))
            # outfile.write("    assert!(l > {0:.10f} && l < {1:.10f});\n"
            #     .format(float(second_line[1])-0.0000000001, float(second_line[1])+0.0000000001))

            outfile.write("    assert!(y > {0:.10f} && y < {1:.10f});\n"
                .format(float(line[4])-0.0000000001, float(line[4])+0.0000000001))
            # outfile.write("    assert!(h > {0:.10f} && h < {1:.10f});\n"
            #     .format(float(second_line[4])-0.0000000001, float(second_line[4])+0.0000000001))

            outfile.write("    assert!(z > {0:.7f} && z < {1:.7f});\n"
                .format(float(line[7])-0.0000025, float(line[7])+0.0000025))
            # outfile.write("    assert!(p > {0:.8f} && p < {1:.8f});\n"
            #     .format(float(second_line[7])-0.00000038, float(second_line[7])+0.00000038))

        elif line[0] == "VSOP87B":
            if current == "vsop87a":
                current = "vsop87b"
                current_planet = ""
                print("Found VSOP87B")
                outfile.write("}\n")
                outfile.close()
                print("Finished VSOP87A")
                outfile = open("tests/vsop87b_tests.rs", 'w')
                outfile.write("extern crate vsop87;\nuse vsop87::*;\n");

            jde = line[2][2:]
            if line[1].lower().replace("-", "_") != current_planet:
                if current_planet != "":
                    outfile.write("}\n")
                current_planet = line[1].lower().replace("-", "_")
                outfile.write("\n#[test]\nfn it_"+ current_planet +"() {\n")
                new_planet = True

        elif current == "vsop87b" and not line[0].startswith("VSOP87") and line[0] != "\x1a":
            if not new_planet:
                outfile.write("\n")
            else:
                new_planet = False

            outfile.write("    let (l, b, r) = vsop87b::"+ current_planet +"("+ jde +");\n\n")

            next(data)
            # try:
            #     second_line = next(data).split()
            # except:
            #     print("No more lines. Last line:")
            #     print(line)

            outfile.write("    assert!(l > {0:.10f} && l < {1:.10f});\n"
                .format(float(line[1])-0.0000000001, float(line[1])+0.0000000001))
            # outfile.write("    assert!(l > {0:.10f} && l < {1:.10f});\n"
            #     .format(float(second_line[1])-0.0000000001, float(second_line[1])+0.0000000001))

            outfile.write("    assert!(b > {0:.10f} && b < {1:.10f});\n"
                .format(float(line[4])-0.0000000001, float(line[4])+0.0000000001))
            # outfile.write("    assert!(h > {0:.10f} && h < {1:.10f});\n"
            #     .format(float(second_line[4])-0.0000000001, float(second_line[4])+0.0000000001))

            outfile.write("    assert!(r > {0:.7f} && r < {1:.7f});\n"
                .format(float(line[7])-0.0000004, float(line[7])+0.0000004))
            # outfile.write("    assert!(p > {0:.8f} && p < {1:.8f});\n"
            #     .format(float(second_line[7])-0.00000038, float(second_line[7])+0.00000038))

        elif line[0] == "VSOP87C":
            if current == "vsop87b":
                current = "vsop87c"
                current_planet = ""
                print("Found VSOP87C")
                outfile.write("}\n")
                outfile.close()
                print("Finished VSOP87B")
                outfile = open("tests/vsop87c_tests.rs", 'w')
                outfile.write("extern crate vsop87;\nuse vsop87::*;\n");

            jde = line[2][2:]
            if line[1].lower().replace("-", "_") != current_planet:
                if current_planet != "":
                    outfile.write("}\n")
                current_planet = line[1].lower().replace("-", "_")
                outfile.write("\n#[test]\nfn it_"+ current_planet +"() {\n")
                new_planet = True

        elif current == "vsop87c" and not line[0].startswith("VSOP87") and line[0] != "\x1a":
            if not new_planet:
                outfile.write("\n")
            else:
                new_planet = False

            outfile.write("    let (x, y, z) = vsop87c::"+ current_planet +"("+ jde +");\n\n")

            next(data)
            # try:
            #     second_line = next(data).split()
            # except:
            #     print("No more lines. Last line:")
            #     print(line)

            outfile.write("    assert!(x > {0:.10f} && x < {1:.10f});\n"
                .format(float(line[1])-0.0000000001, float(line[1])+0.0000000001))
            # outfile.write("    assert!(l > {0:.10f} && l < {1:.10f});\n"
            #     .format(float(second_line[1])-0.0000000001, float(second_line[1])+0.0000000001))

            outfile.write("    assert!(y > {0:.10f} && y < {1:.10f});\n"
                .format(float(line[4])-0.0000000001, float(line[4])+0.0000000001))
            # outfile.write("    assert!(h > {0:.10f} && h < {1:.10f});\n"
            #     .format(float(second_line[4])-0.0000000001, float(second_line[4])+0.0000000001))

            outfile.write("    assert!(z > {0:.8f} && z < {1:.8f});\n"
                .format(float(line[7])-0.0000003, float(line[7])+0.0000003))
            # outfile.write("    assert!(p > {0:.8f} && p < {1:.8f});\n"
            #     .format(float(second_line[7])-0.00000038, float(second_line[7])+0.00000038))

        elif line[0] == "VSOP87D":
            if current == "vsop87c":
                current = "vsop87d"
                current_planet = ""
                print("Found VSOP87D")
                outfile.write("}\n")
                outfile.close()
                print("Finished VSOP87C")
                outfile = open("tests/vsop87d_tests.rs", 'w')
                outfile.write("extern crate vsop87;\nuse vsop87::*;\n");

            jde = line[2][2:]
            if line[1].lower().replace("-", "_") != current_planet:
                if current_planet != "":
                    outfile.write("}\n")
                current_planet = line[1].lower().replace("-", "_")
                outfile.write("\n#[test]\nfn it_"+ current_planet +"() {\n")
                new_planet = True

        elif current == "vsop87d" and not line[0].startswith("VSOP87") and line[0] != "\x1a":
            if not new_planet:
                outfile.write("\n")
            else:
                new_planet = False

            outfile.write("    let (l, b, r) = vsop87d::"+ current_planet +"("+ jde +");\n\n")

            next(data)
            # try:
            #     second_line = next(data).split()
            # except:
            #     print("No more lines. Last line:")
            #     print(line)

            outfile.write("    assert!(l > {0:.10f} && l < {1:.10f});\n"
                .format(float(line[1])-0.0000000001, float(line[1])+0.0000000001))
            # outfile.write("    assert!(l > {0:.10f} && l < {1:.10f});\n"
            #     .format(float(second_line[1])-0.0000000001, float(second_line[1])+0.0000000001))

            outfile.write("    assert!(b > {0:.10f} && b < {1:.10f});\n"
                .format(float(line[4])-0.0000000001, float(line[4])+0.0000000001))
            # outfile.write("    assert!(h > {0:.10f} && h < {1:.10f});\n"
            #     .format(float(second_line[4])-0.0000000001, float(second_line[4])+0.0000000001))

            outfile.write("    assert!(r > {0:.8f} && r < {1:.8f});\n"
                .format(float(line[7])-0.00000038, float(line[7])+0.00000038))
            # outfile.write("    assert!(p > {0:.8f} && p < {1:.8f});\n"
            #     .format(float(second_line[7])-0.00000038, float(second_line[7])+0.00000038))

        elif line[0] == "VSOP87E":
            if current == "vsop87d":
                current = "vsop87e"
                current_planet = ""
                print("Found VSOP87E")
                outfile.write("}\n")
                outfile.close()
                print("Finished VSOP87D")
                outfile = open("tests/vsop87e_tests.rs", 'w')
                outfile.write("extern crate vsop87;\nuse vsop87::*;\n");

            jde = line[2][2:]
            if line[1].lower().replace("-", "_") != current_planet:
                if current_planet != "":
                    outfile.write("}\n")
                current_planet = line[1].lower().replace("-", "_")
                outfile.write("\n#[test]\nfn it_"+ current_planet +"() {\n")
                new_planet = True

        elif current == "vsop87e" and not line[0].startswith("VSOP87") and line[0] != "\x1a":
            if not new_planet:
                outfile.write("\n")
            else:
                new_planet = False

            outfile.write("    let (x, y, z) = vsop87e::"+ current_planet +"("+ jde +");\n\n")

            next(data)
            # try:
            #     second_line = next(data).split()
            # except:
            #     print("No more lines. Last line:")
            #     print(line)

            outfile.write("    assert!(x > {0:.10f} && x < {1:.10f});\n"
                .format(float(line[1])-0.0000000001, float(line[1])+0.0000000001))
            # outfile.write("    assert!(l > {0:.10f} && l < {1:.10f});\n"
            #     .format(float(second_line[1])-0.0000000001, float(second_line[1])+0.0000000001))

            outfile.write("    assert!(y > {0:.10f} && y < {1:.10f});\n"
                .format(float(line[4])-0.0000000001, float(line[4])+0.0000000001))
            # outfile.write("    assert!(h > {0:.10f} && h < {1:.10f});\n"
            #     .format(float(second_line[4])-0.0000000001, float(second_line[4])+0.0000000001))

            outfile.write("    assert!(z > {0:.7f} && z < {1:.7f});\n"
                .format(float(line[7])-0.000002, float(line[7])+0.000002))
            # outfile.write("    assert!(p > {0:.8f} && p < {1:.8f});\n"
            #     .format(float(second_line[7])-0.00000038, float(second_line[7])+0.00000038))

    outfile.write("}\n")
    outfile.close()
    print("Finished VSOP87E")
