from datetime import datetime
import sys, tempfile, csv, subprocess, pandas, pprint, random, time

from gen_people import random_row


argc = len(sys.argv)

PROGRAM_NAME = sys.argv[1] if argc > 1 else None
assert PROGRAM_NAME, "Must provide a path to the program under test. Aborting."

ITERATIONS = int(sys.argv[2]) if argc > 2 else 100
NAME_LEN = int(sys.argv[3]) if argc > 3 else 10

SEPARATORS = [",", "|", " "]
NUM_INPUT_FILES = 10
DATE_FORMAT = "%-m/%-d/%Y"

LAST_NAME = 0
FIRST_NAME = 1
EMAIL = 2
FAVORITE_COLOR = 3
DOB = 4

HEADER_ROW = ["last_name", "first_name", "email", "favorite_color", "dob"]


test_cases = [
    {
        "fields": [("favorite_color", "asc"), ("last_name", "asc")],
        "separator": random.choice(SEPARATORS),
        "has_header": bool(random.randint(0,1))
    },
    {
        "fields": [("dob", "asc")],
        "separator": random.choice(SEPARATORS),
        "has_header": bool(random.randint(0,1))
    },
    {
        "fields": [("last_name", "desc")],
        "separator": random.choice(SEPARATORS),
        "has_header": bool(random.randint(0,1))
    }
]


def incoming(row):
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
    has_header = case["has_header"]

    class rows:
        us = []
        them = []

    # generate a bunch of tempfiles with rows of data
    for i in range(0, NUM_INPUT_FILES):
        tmp_file = tempfile.NamedTemporaryFile(prefix="homework-") #, delete=False)
        tmp_files.append(tmp_file)

        with open(tmp_file.name, "w") as tmp:
            writer = csv.writer(tmp, delimiter=sep)
            if has_header:
                writer.writerow(HEADER_ROW)

            for i in range(0, ITERATIONS):
                writer.writerow(random_row(DATE_FORMAT, NAME_LEN))

    # read into memory what was just written
    for f in tmp_files:
        with open(f.name, "r") as tmp:
            reader = csv.reader(tmp, delimiter=sep)
            if has_header:
                next(reader, None)
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
    argv = [PROGRAM_NAME, "-S{}".format(sep)]
    if has_header:
        argv += ["-E"]
    argv += [arg for pair in field_args for arg in pair]
    argv += ["--"] + [x.name for x in tmp_files]

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

            # Only check the fields whose values are guaranteed to be deterministically sorted
            if j in (globals()[n.upper()] for n in field_names) and us != them:
                failures.append({
                    "row": i, "column": j, "us": us, "them": them
                })

    report = {
        "testcase": case,
        "cmd": " ".join(argv),
        "failures": len(failures),
    }

    if failures:
        report["failure-samples"] = failures[:3]

    pprint.pprint(report)

sys.exit(len(failures) > 1)
    