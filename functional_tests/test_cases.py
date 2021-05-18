import random


SEPARATORS = [",", "|", " "]
FIELDS = ["last_name", "first_name", "email", "favorite_color", "dob"]
DIRECTIONS = ["asc", "desc"]


## Example:
# test_cases = [
#     {
#         "fields": [("favorite_color", "asc"), ("last_name", "asc")],
#         "separator": "random.choice(SEPARATORS)",
#         "has_header": bool(random.randint(0,1))
#     },
#     {
#         "fields": [("dob", "asc")],
#         "separator": random.choice(SEPARATORS),
#         "has_header": bool(random.randint(0,1))
#     },
#     {
#         "fields": [("last_name", "desc")],
#         "separator": random.choice(SEPARATORS),
#         "has_header": bool(random.randint(0,1))
#     }
# ]


def random_fields(max = len(FIELDS)):
    fields = []
    how_many = random.randint(1, max)

    for i in range(0, how_many):
        fields.append(
            (random.choice(FIELDS), random.choice(DIRECTIONS))
        )

    return fields


def generate_random_test_cases(iterations, max_fields = 2):
    test_cases = []

    for i in range(0, iterations):
        test_cases.append({
            "fields": random_fields(max_fields),
            "separator": random.choice(SEPARATORS),
            "has_header": bool(random.randint(0, 1))
        })

    return test_cases