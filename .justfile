build_all:
    ./build.sh
build:
    cd build;make -j;

build_release_all:
    ./build_release.sh
build_release:
    cd build_release;make -j;

clean:
    rm -rf ./build
clean_release:
    rm -rf ./build_release

run_release:
    bash ./brun_release.sh