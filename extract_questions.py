import sys
import os
import argparse

# Get the questions and answers from the Pufendorf chatbot.

parser = argparse.ArgumentParser()
parser.add_argument("-f", "--filename", help="Filename of log file.", default="lucrisbot.log")
args = parser.parse_args()

in_q = False
in_a = False
with open(args.filename, "r") as f:
    for line in f:
        line = line.strip()
        bits = line.split()
        if len(bits) > 5:
            bits = bits[5:]
            if bits[0] == "USER:":
                #print("-" * 40)
                in_q = True
                print("")
                print("QUESTION")
                print(" ".join(bits[1:]))
                continue
            if in_q:
                if bits[0] == "FULL":
                    in_q = False
                if bits[0] == "TEMP:":
                    in_a = True
                    in_q = False
                    print("ANSWER")
                    continue
            if not in_q:
                if bits[0] == "IGNORE":
                    in_q = True
            if in_a or in_q:
                if bits[0].startswith("2025"): # Remove date
                    in_q = False
                    in_a = False
            if in_q:
                #print("\t", " ".join(bits))
                pass
            if in_a:
                if line.startswith("2025"):
                    print(" ".join(bits))
                else:
                    print(line)
            
            
