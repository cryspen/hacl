#    Copyright 2022 Cryspen Sarl
#
#    Licensed under the Apache License, Version 2.0 or MIT.
#    * http://www.apache.org/licenses/LICENSE-2.0
#    * http://opensource.org/licenses/MIT

import json
import os
import platform
import re
import shutil
import subprocess
from glob import glob
from os.path import join

from tools.utils import argument, cmake_config, dep_config, json_config, subcommand


@subcommand()
def configure(args):
    """Only run configure to generate the config.cmake and dep_config.json"""
    source_dir = "src"
    include_dir = "include"
    config = Config(json_config(), source_dir, include_dir)
    config.write_cmake_config(cmake_config())
    config.write_dep_config(dep_config())


class Config:
    def dependencies(self, source_dir, algorithm, source_file):
        """Collect dependencies for a given c file

        Use `$CC -MM` to collect dependencies for a given c file assuming header
        and source files are named the same.
        """
        # With old compilers like GCC 4.8 we have to set -march=native for this
        # to work.
        copmiler_version = subprocess.run(
            self.compiler + " --version", stdout=subprocess.PIPE, shell=True, check=True
        )
        stdout = copmiler_version.stdout.decode("utf-8")
        args = ""
        if "4.8" in stdout and "gcc" in stdout:
            processor = platform.processor()
            if "x86" in processor:
                args = " -march=native "
        if "clang" in stdout and self.target:
            # add self.target as -target but only when the compiler is clang
            # and we don't do android
            if not "android" in self.target:
                args += " -target " + self.target
            if "s390x" in self.target:
                args += " -mzvector -march=z14"
        # Build dependency graph
        # FIXME: read include paths and CC from config.json
        includes = "-I " + " -I ".join(self.include_paths)
        result = subprocess.run(
            self.compiler
            + args
            + " "
            + includes
            + " -I"
            + join(source_dir, "internal")
            + " -MM "
            + join(source_dir, source_file),
            stdout=subprocess.PIPE,
            shell=True,
            check=True,
        )
        stdout = result.stdout.decode("utf-8")

        files = []
        for line in stdout.splitlines():
            # Remove object file and the c file itself
            first_line_search = "(\w*).o: " + re.escape(join(source_dir, "(\w*).c"))
            line = re.sub(first_line_search, "", line)
            line = line.strip()
            line = line.split(" ")
            try:
                line.remove("\\")
            except:
                # This is fine
                pass
            files.extend(line)

        # Get all source files in source_dir
        source_files = glob(join(source_dir, "*.c"))
        # remove source_dir and .c
        source_files = list(map(lambda s: s[len(source_dir) + 1 : -2], source_files))

        # Now let's collect the c files from the included headers
        # This adds all files without looking at the feature requirements into deps.
        deps = []
        includes = []
        for include in files:
            # Get the file name from the path
            file_name = os.path.splitext(os.path.basename(include))[0]
            # Only add the dependency if there's a corresponding source file.
            for s in source_files:
                if s.lower() == file_name.lower():
                    deps.append(join(source_dir, s + ".c"))
            # We take all includes though
            if include.endswith(".h"):
                includes.append(include)
        return deps, includes

    def __init__(
        self,
        config_file,
        source_dir,
        include_dir,
        algorithms=[],
        compiler="clang",
        target=None,
    ):
        """Read the build config from the json file"""
        print(" [mach] Using %s to configure ..." % (config_file))
        if len(algorithms) != 0:
            print(" [mach]   enabling %s" % " ".join(algorithms))

        # read file
        with open(config_file, "r") as f:
            data = f.read()

        self.compiler = compiler
        self.target = target

        # parse file
        self.config = json.loads(data)
        self.hacl_files = self.config["hacl_sources"]
        self.evercrypt_files = self.config["evercrypt_sources"]
        self.vale_files = self.config["vale_sources"]
        self.libcrux_files = self.config["libcrux_sources"]
        self.tests = self.config["tests"]
        self.benchmarks = self.config["benchmarks"]

        self.include_paths = [include_dir]
        # We need the config.h generated by CMake
        self.include_paths.append("build")
        # Set kremlin as include paths
        self.include_paths.extend(self.config["karamel_include_paths"])
        # If vale is compiled add the include path
        if len(self.vale_files) != 0:
            self.include_paths.extend(self.config["vale_include_paths"])
        # If libcrux is compiled add the include path
        if len(self.libcrux_files) != 0:
            self.include_paths.extend(self.config["libcrux_include_paths"])

        # If the build directory is empty, copy the `default_config.h` there to
        # make the dependency analysis work.
        if not os.path.isfile(join("build", "config.h")):
            shutil.copyfile(
                join("config", "default_config.h"), join("build", "config.h")
            )

        # Filter algorithms in hacl_files
        # In the default case (empty list of algorithms) we don't do anything.
        if len(algorithms) != 0:
            # Check if the algorithms are actually valid
            for alg in algorithms:
                if not alg in self.hacl_files:
                    print(" [mach] ! Unsupported algorithm requested: %s" % alg)
                    exit(1)
            for a, _ in list(self.hacl_files.items()):
                if not a in algorithms:
                    del self.hacl_files[a]
            for a, _ in list(self.evercrypt_files.items()):
                if not a in algorithms:
                    del self.evercrypt_files[a]
            for a, _ in list(self.tests.items()):
                if not a in algorithms:
                    del self.tests[a]
            for a, _ in list(self.vale_files.items()):
                if not a in algorithms and a != "std":
                    del self.vale_files[a]

        # Collect dependencies for the hacl files.
        self.hacl_compile_feature = {}
        self.hacl_includes = []
        for a in self.hacl_files:
            for source_file in self.hacl_files[a]:
                files, includes = self.dependencies(source_dir, a, source_file["file"])
                self.hacl_includes.extend(
                    includes if type(includes) == list else [includes]
                )
                feature = source_file["features"].replace(",", "_")
                if feature in self.hacl_compile_feature:
                    self.hacl_compile_feature[feature].extend(
                        files if type(files) == list else [files]
                    )
                else:
                    # Add the new feature dependency
                    self.hacl_compile_feature[feature] = (
                        files if type(files) == list else [files]
                    )
        # Remove files that require additional features from hacl_compile_files
        for feature in self.hacl_compile_feature:
            if feature != "std":
                # Filter all feature files to remove std files.
                self.hacl_compile_feature[feature] = [
                    file
                    for file in self.hacl_compile_feature[feature]
                    if file not in self.hacl_compile_feature["std"]
                ]

        # Flatten test sources
        self.test_sources = [
            f for files in [self.tests[b] for b in self.tests] for f in files
        ]

        # Flatten benchmark sources
        self.benchmark_sources = [
            f for files in [self.benchmarks[b] for b in self.benchmarks] for f in files
        ]

        # Flatten vale files into a single list for each platform.
        # This is all or nothing.
        platforms = {}
        for algorithm in self.vale_files:
            for p in self.vale_files[algorithm]:
                if p in platforms:
                    platforms[p].extend(self.vale_files[algorithm][p])
                else:
                    platforms[p] = self.vale_files[algorithm][p]
        for p in platforms:
            platforms[p] = [join("vale", "src", f) for f in platforms[p]]
        self.vale_files = platforms

        # Flatten libcrux sources
        libcrux_files_flattened = []
        for _, impls in self.libcrux_files.items():
            libcrux_files_flattened.extend(impl["file"] for impl in impls)
        self.libcrux_files = [
            join("libcrux", "src", f) for f in libcrux_files_flattened
        ]

        # Evercrypt has feature detection and we don't disable anything.
        self.evercrypt_compile_files = []
        for a in self.evercrypt_files:
            for source_file in self.evercrypt_files[a]:
                files, includes = self.dependencies(source_dir, a, source_file)
                self.evercrypt_compile_files.extend(files)
                self.hacl_includes.extend(
                    includes if type(includes) == list else [includes]
                )

        # Remove duplicates from all lists
        for k in self.hacl_compile_feature:
            self.hacl_compile_feature[k] = list(
                dict.fromkeys(self.hacl_compile_feature[k])
            )
        self.evercrypt_compile_files = list(dict.fromkeys(self.evercrypt_compile_files))
        self.hacl_includes = list(dict.fromkeys(self.hacl_includes))
        # Drop Hacl_ files from evercrypt
        self.evercrypt_compile_files = [
            f for f in self.evercrypt_compile_files if "Hacl_" not in f
        ]
        self.hacl_compile_feature["std"].extend(self.evercrypt_compile_files)

        # Remove duplicates across <feature>_vale and <feauture>
        for f1 in self.hacl_compile_feature:
            for f2 in self.hacl_compile_feature:
                if f1 != f2 and f1.endswith("_vale"):
                    self.hacl_compile_feature[f1] = [
                        i
                        for i in self.hacl_compile_feature[f1]
                        if i not in self.hacl_compile_feature[f2]
                    ]

        # We don't want internal excludes to be installed.
        self.public_includes = [
            file
            for file in self.hacl_includes
            if join("internal", os.path.basename(file)) not in file
        ]

    def write_cmake_config(self, cmake_config):
        print(" [mach] Writing cmake config to %s ..." % (cmake_config))
        # cmake wants the unix style for paths apparently
        with open(cmake_config, "w") as out:
            for a in self.hacl_compile_feature:
                out.write(
                    "set(SOURCES_%s\n\t%s\n)\n"
                    % (
                        a,
                        "\n\t".join(
                            join("${PROJECT_SOURCE_DIR}", f)
                            for f in self.hacl_compile_feature[a]
                        ).replace("\\", "/"),
                    )
                )

            out.write(
                "set(INCLUDES\n\t%s\n)\n"
                % "\n\t".join(
                    join("${PROJECT_SOURCE_DIR}", a) for a in self.hacl_includes
                ).replace("\\", "/")
            )

            out.write(
                "set(PUBLIC_INCLUDES\n\t%s\n)\n"
                % "\n\t".join(
                    join("${PROJECT_SOURCE_DIR}", a) for a in self.public_includes
                ).replace("\\", "/")
            )

            out.write(
                "set(ALGORITHMS\n\t%s\n)\n"
                % "\n\t".join(a for a in self.hacl_files).replace("\\", "/")
            )

            out.write(
                "set(INCLUDE_PATHS\n\t%s\n)\n"
                % "\n\t".join(
                    join("${PROJECT_SOURCE_DIR}", p) for p in self.include_paths
                ).replace("\\", "/")
            )

            out.write(
                "set(TEST_SOURCES\n\t%s\n)\n"
                % (
                    "\n\t".join(
                        join("${PROJECT_SOURCE_DIR}", "tests", f)
                        for f in self.test_sources
                    ).replace("\\", "/")
                )
            )

            out.write(
                "set(BENCHMARK_SOURCES\n\t%s\n)\n"
                % (
                    "\n\t".join(
                        join("${PROJECT_SOURCE_DIR}", "benchmarks", f)
                        for f in self.benchmark_sources
                    ).replace("\\", "/")
                )
            )

            for os in self.vale_files:
                out.write(
                    "set(VALE_SOURCES_%s\n\t%s\n)\n"
                    % (
                        os,
                        "\n\t".join(
                            join("${PROJECT_SOURCE_DIR}", f)
                            for f in self.vale_files[os]
                        ).replace("\\", "/"),
                    )
                )

            for os in self.libcrux_files:
                out.write(
                    "set(LIBCRUX_SOURCES\n\t%s\n)\n"
                    % (
                        "\n\t".join(
                            join("${PROJECT_SOURCE_DIR}", f) for f in self.libcrux_files
                        ).replace("\\", "/"),
                    )
                )

            out.write(
                "set(ALGORITHM_TEST_FILES\n\t%s\n)\n"
                % "\n\t".join("TEST_FILES_" + a for a in self.tests).replace("\\", "/")
            )
            for a in self.tests:
                out.write(
                    "set(TEST_FILES_%s\n\t%s\n)\n"
                    % (a, "\n\t".join(f for f in self.tests[a]).replace("\\", "/"))
                )

    def dep_config(self):
        print(" [mach] Collecting files and dependencies ...")
        includes = [
            include
            for include in self.hacl_includes
            if not include.startswith("kremlin") and not include.startswith("vale")
        ]
        vale_includes = [
            include for include in self.hacl_includes if include.startswith("vale")
        ]
        kremlin_includes = [
            include for include in self.hacl_includes if include.startswith("kremlin")
        ]
        libcrux_includes = [
            include for include in self.include_paths if include.startswith("libcrux")
        ]
        return {
            "sources": self.hacl_compile_feature,
            "includes": includes,
            "kremlin_includes": kremlin_includes,
            "vale_sources": self.vale_files,
            "vale_includes": vale_includes,
            "libcrux_sources": self.libcrux_files,
            "libcrux_includes": libcrux_includes,
        }

    def write_dep_config(self, dep_config):
        config = self.dep_config()
        json_data = json.dumps(config, indent=4)
        with open(dep_config, "w") as outfile:
            outfile.write(json_data)

    def source_files(self):
        """Get a list of all source files in the config."""
        out = []
        # FIXME
        # for a in self.hacl_compile_files:
        #     out.extend(self.hacl_compile_files[a])
        for a in self.evercrypt_compile_files:
            out.extend(self.evercrypt_compile_files[a])
        return out

    # TODO: we first have to create a list of headers
    def header_files(self):
        """Get a list of all header files in the config."""
        pass
