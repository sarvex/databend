#!/usr/bin/env python3

import os
import sys
import signal

CURDIR = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, os.path.join(CURDIR, "../../../helpers"))

from native_client import NativeClient
from native_client import prompt

log = None
endpoint_url = os.environ.get("STORAGE_S3_ENDPOINT_URL")

with NativeClient(name="client1>") as client1:
    client1.expect(prompt)

    client1.send("drop table if exists ontime200;")
    client1.expect(prompt)

    client1.send("drop stage if exists named_external_stage;")
    client1.expect(prompt)

    create_sql_f = os.path.join(CURDIR, "../ddl/ontime.sql")
    with open(create_sql_f, "r") as read:
        create_sql = read.read().replace("ontime", "ontime200")
    client1.send(create_sql)
    client1.expect(prompt)

    client1.send(
        f"CREATE STAGE named_external_stage url = 's3://testbucket/admin/data/' connection=(aws_key_id='minioadmin' aws_secret_key='minioadmin' endpoint_url='{endpoint_url}');"
    )
    client1.expect(prompt)

    client1.send("SET max_block_size = 50;")
    client1.expect(prompt)

    client1.send(
        "copy into ontime200 from @named_external_stage  PATTERN = 'ontime_200.csv$' FILE_FORMAT = (type = CSV field_delimiter = ','  record_delimiter = '\n' skip_header = 1);"
    )
    client1.expect(prompt)

    client1.send("select count() from ontime200;")
    client1.expect("199")

    client1.send("drop table ontime200;")
    client1.expect(prompt)
