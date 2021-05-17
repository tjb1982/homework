from datetime import datetime
import sys, tempfile, csv, subprocess, pandas, pprint, random

from gen_people import random_row


iterations = int(sys.argv[1]) if len(sys.argv) > 1 else 100
sep = sys.argv[2] if len(sys.argv) > 2 else ","
name_len = int(sys.argv[3]) if len(sys.argv) > 3 else 10
print_header = len(sys.argv) > 4 and sys.argv[4].lower().startswith("t")

SEPARATORS = [",", "|", " "]
PROGRAM_NAME = "target/release/homework"
NUM_INPUT_FILES = 10
DATE_FORMAT = "%-m/%-d/%Y"

LAST_NAME = 0
FIRST_NAME = 1
EMAIL = 2
FAVORITE_COLOR = 3
DOB = 4


test_cases = [
    {
        "fields": [("favorite_color", "asc"), ("last_name", "asc")],
        "separator": random.choice(SEPARATORS)
    },
    {
        "fields": [("dob", "asc")],
        "separator": random.choice(SEPARATORS)
    },
    {
        "fields": [("last_name", "desc")],
        "separator": random.choice(SEPARATORS)
    }
]


def incoming(row, pr=False):
    if pr:
        print(row)
    return [
        row[LAST_NAME],
        row[FIRST_NAME],
        row[EMAIL],
        row[FAVORITE_COLOR],
        datetime.strptime(row[DOB], DATE_FORMAT.replace("-", ""))
    ]

failures = []

for case in test_cases:

    tmp_files = []
    sep = case["separator"]

    class rows:
        us = []
        them = []

    # generate a bunch of tempfiles with rows of data
    for i in range(0, NUM_INPUT_FILES):
        tmp_file = tempfile.NamedTemporaryFile(prefix="homework-") #, delete=False)
        tmp_files.append(tmp_file)

        with open(tmp_file.name, "w") as tmp:
            for i in range(0, iterations):
                writer = csv.writer(tmp, delimiter=sep)
                writer.writerow(random_row(DATE_FORMAT, name_len))

    # read into memory what was just written
    for f in tmp_files:
        with open(f.name, "r") as tmp:
            reader = csv.reader(tmp, delimiter=sep)
            rows.us += [incoming(row) for row in reader]

    # sort it in memory like the program under test should
    df =  pandas.DataFrame(rows.us)

    field_names, directions = zip(*case["fields"])
    sorted_values = df.sort_values(
        by=[globals()[x.upper()] for x in field_names],
        ascending=[x == "asc" for x in directions]
    )
    rows.us = sorted_values.agg(list, 1).tolist()
    ##########

    # run the program under test over the same tempfiles in a subprocess
    field_args = (("-f", field_name, "-d", direction) for field_name, direction in case["fields"])
    argv = [PROGRAM_NAME, "-S{}".format(sep)] + \
        [arg for pair in field_args for arg in pair] + \
        ["--"] + [x.name for x in tmp_files]

    result = subprocess.run(argv, stdout=subprocess.PIPE)

    # read the stdout from that subprocess and parse it with csv.reader
    stdout = result.stdout.decode("utf-8")
    reader = csv.reader(stdout.split("\n"), delimiter=sep)
    rows.them = [incoming(row) for row in reader if row]
    ##########

    # compare the us vs. them
    for i, row in enumerate(rows.us):
        for j, us in enumerate(row):
            them = rows.them[i][j]

            if us != them:
                failures.append({
                    "row": i, "column": j, "us": us, "them": them
                })

    report = {
        "testcase": case,
        "argv": argv,
        "failures": len(failures),
    }

    if failures:
        report["failure-samples"] = failures[:3]

    pprint.pprint(report)

sys.exit(len(failures) > 1)
    