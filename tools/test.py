
#    Copyright 2022 Cryspen Sarl
#
#    Licensed under the Apache License, Version 2.0 (the "License");
#    you may not use this file except in compliance with the License.
#    You may obtain a copy of the License at
#
#        http://www.apache.org/licenses/LICENSE-2.0
#
#    Unless required by applicable law or agreed to in writing, software
#    distributed under the License is distributed on an "AS IS" BASIS,
#    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#    See the License for the specific language governing permissions and
#    limitations under the License.

import json
import os
import re
import subprocess
import sys
from tools.configure import Config
from tools.ocaml import test_ocaml
from tools.utils import subcommand, argument, cli, subparsers, mprint as print, binary_path, json_config

from os.path import join
from pathlib import Path


def run_tests(tests, bin_path, test_args=[], algorithms=[]):
    print("Running tests ...")
    if not os.path.exists(binary_path(bin_path)):
        print("! Nothing is built! Please build first. Aborting!")
        exit(1)
    os.chdir(binary_path(bin_path))
    my_env = dict(os.environ)
    my_env["TEST_DIR"] = join(os.getcwd(), "..", "..", "tests")
    for algorithm in tests:
        for test in tests[algorithm]:
            test_name = os.path.splitext(test)[0]
            if len(algorithms) == 0 or test_name in algorithms or algorithm in algorithms:
                file_name = Path(test).stem
                if sys.platform == "win32":
                    file_name += ".exe"
                if not os.path.exists(file_name):
                    print("! Test '%s' doesn't exist. Aborting!" %
                          (file_name))
                    print("   Running this test requires a build first.")
                    print("   See mach.py build --help")
                    exit(1)
                test_cmd = [join(".", file_name)]
                test_cmd.extend(test_args)
                print(" ".join(test_cmd))
                subprocess.run(test_cmd, check=True, shell=True, env=my_env)

# TODO: add arguments (pass through gtest arguments and easy filters)


@subcommand([argument("-a", "--algorithms",
                      help="The algorithms to test.", type=str),
             argument("-l", "--language",
                      help="Language bindings to test.", type=str),
             argument("-v", "--verbose", help="Make tests verbose.",
                      action='store_true')])
def test(args):
    """Test HACL*
    """
    if args.language:
        # We ignore algorithms here. Just run the language bindings' tests.
        if args.language == "ocaml":
            test_ocaml()
            exit(0)
        elif args.language == "rust":
            subprocess.run(['cargo', 'test', '--manifest-path=rust/Cargo.toml'], check=True)
            exit(0)
        else:
            print("Unknown language binding %s. Please see --help for supported bindings" % (args.l))
            exit(1)

    algorithms = []
    if args.algorithms:
        algorithms = re.split(r"\W+", args.algorithms)

    # read file
    with open(json_config(), 'r') as f:
        data = f.read()

    # parse file
    config = json.loads(data)
    # config = Config(json_config(), algorithms=algorithms)
    run_tests(config['tests'], "Debug", algorithms=algorithms)
