from datetime import datetime
import sys, tempfile, csv, subprocess, pandas, json

from gen_people import random_row
from test_cases import generate_random_test_cases


NUM_INPUT_FILES = 10
DATE_FORMAT = "%-m/%-d/%Y"

LAST_NAME = 0
FIRST_NAME = 1
EMAIL = 2
FAVORITE_COLOR = 3
DOB = 4

HEADER_ROW = ["last_name", "first_name", "email", "favorite_color", "dob"]


def incoming(row):
    return [
        row[LAST_NAME],
        row[FIRST_NAME],
        row[EMAIL],
        row[FAVORITE_COLOR],
        datetime.strptime(row[DOB], DATE_FORMAT.replace("-", ""))
    ]


def generate_tempfiles(num_files, iterations, sep, has_header):
    tmp_files = []

    # generate a bunch of tempfiles with rows of data
    for _ in range(0, num_files):
        tmp_file = tempfile.NamedTemporaryFile(prefix="homework-") #, delete=False)
        tmp_files.append(tmp_file)

        with open(tmp_file.name, "w") as tmp:
            writer = csv.writer(tmp, delimiter=sep)
            if has_header:
                writer.writerow(HEADER_ROW)

            for i in range(0, iterations):
                writer.writerow(random_row(DATE_FORMAT))

    return tmp_files


def read_tmp_files_into_rows(tmp_files, sep, has_header):
    rows = []

    # read into memory what was just written
    for f in tmp_files:
        with open(f.name, "r") as tmp:
            reader = csv.reader(tmp, delimiter=sep)
            if has_header:
                next(reader, None)
            rows += [incoming(row) for row in reader]

    return rows


def sorted_rows(rows, field_names, directions):
    # sort it in memory like the program under test should
    df =  pandas.DataFrame(rows)

    sorted_values = df.sort_values(
        by=[globals()[x.upper()] for x in field_names],
        ascending=[x == "asc" for x in directions]
    )

    return sorted_values.agg(list, 1).tolist()


def run_program_under_test(program, tmp_files, sep, has_header, fields):
    # run the program under test over the same tempfiles in a subprocess
    field_args = (("-f", field_name, "-d", direction) for field_name, direction in fields)
    argv = [program, "-S{}".format(sep)]
    if has_header:
        argv += ["-E"]
    argv += [arg for pair in field_args for arg in pair]
    argv += ["--"] + [x.name for x in tmp_files]

    return subprocess.run(argv, stdout=subprocess.PIPE)


def progress(passing = True):
    sys.stderr.write("." if passing else "\U0001D350")
    sys.stderr.flush()


def compare_model_with_program_under_test(program_name, rows_per_file, case):
        fields = case["fields"]
        field_names, directions = zip(*fields)
        sep = case.get("separator", ",")
        has_header = case.get("has_header")
        failures = []

        class rows:
            us = []
            them = []

        tmp_files = generate_tempfiles(
            NUM_INPUT_FILES,
            rows_per_file,
            sep,
            has_header)

        rows.us = sorted_rows(
            read_tmp_files_into_rows(tmp_files, sep, has_header),
            field_names,
            directions)

        result = run_program_under_test(
            program_name,
            tmp_files,
            sep,
            has_header, fields)

        argv = result.args

        # read the stdout from that subprocess and parse it with csv.reader
        stdout = result.stdout.decode("utf-8")
        reader = csv.reader(stdout.split("\n"), delimiter=sep)
        rows.them = [incoming(row) for row in reader if row]

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

        progress(not failures)

        if failures:
            report["failure-samples"] = failures[:3]

        return report, failures


if __name__ == "__main__":

    argc = len(sys.argv)

    PROGRAM_NAME = sys.argv[1] if argc > 1 else None
    assert PROGRAM_NAME, "Must provide a path to the program under test. Aborting."

    ITERATIONS = int(sys.argv[2]) if argc > 2 else 10
    ROWS_PER_FILE = int(sys.argv[3]) if argc > 3 else 100

    failures = []
    reports = []

    for case in generate_random_test_cases(ITERATIONS, 2):
        report, failures = compare_model_with_program_under_test(
            PROGRAM_NAME, ROWS_PER_FILE, case)
        reports.append(report)
        failures += failures

    sys.stderr.write("\n")
    print(json.dumps(reports))
    sys.exit(1 if failures else 0)
