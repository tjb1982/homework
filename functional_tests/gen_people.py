import string, random
from datetime import datetime, timedelta


colors = ["red", "green", "blue", "yellow", "orange", "violet", "white", "black", "indigo"]
tlds = ["com", "gov", "org", "edu", "biz", "co.uk", "dk", "sv", "se", "de"]


def random_name(length = 10):
    return "{}{}".format(
        random.choice(string.ascii_uppercase),
        "".join(random.choice(string.ascii_lowercase) for i in range(0, length - 1))
    )


def random_email(first, last):
    return "{}.{}@{}.{}".format(
        first,
        last,
        random_name(),
        random.choice(tlds)
    )


def random_color():
    return random.choice(colors)


def random_date(fmt):
    start = datetime.now()
    end = start + timedelta(days=365*-80)
    random_date = start + (end - start) * random.random()

    return random_date.strftime(fmt)


def random_row(fmt, name_len = 10):
    first, last = random_name(name_len), random_name(name_len)

    return [
        last,
        first,
        random_email(first, last),
        random_color(),
        random_date(fmt)
    ]
