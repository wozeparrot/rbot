local_dir := $(dir $(lastword $(MAKEFILE_LIST)))

.PHONY: wpilib_compile cp_libs cp_headers update_submod a-bot_clean wpilib_clean ni_clean

wpilib_compile: local_dir := $(local_dir)
wpilib_compile: update_submod
	cd $(local_dir)allwpilib;./gradlew :hal:halLinuxathenaReleaseSharedLibrary --console=plain --no-scan

update_submod: local_dir := $(local_dir)
update_submod:
	git submodule sync
	git submodule update --init --remote --merge --recursive

cp_libs: local_dir := $(local_dir)
cp_libs: wpilib_compile
	cp -v $(local_dir)allwpilib/hal/build/libs/hal/shared/linuxathena/release/*.so $(local_dir)libs/
	cp -v $(local_dir)allwpilib/wpiutil/build/libs/wpiutil/shared/linuxathena/release/*.so $(local_dir)libs/
	
	cp -v $(local_dir)ni-libraries/src/lib/chipobject/* $(local_dir)libs/
	cp -v $(local_dir)ni-libraries/src/lib/netcomm/* $(local_dir)libs/

	cd $(local_dir)libs && bash -c 'pwd; for i in *.so.*; do mv -i "$$i" "$${i%.so.*}.so"; done'

cp_headers: local_dir := $(local_dir)
cp_headers: update_submod wpilib_compile
	cp -R -v $(local_dir)allwpilib/hal/src/main/native/include/hal/ $(local_dir)headers/
	cp -R -v $(local_dir)allwpilib/hal/build/generated/headers/hal/ $(local_dir)headers/

	cp -R -v $(local_dir)allwpilib/wpiutil/src/main/native/include/* $(local_dir)headers/

	cp -R -v $(local_dir)allwpilib/ntcore/src/main/native/include/* $(local_dir)headers/

	cp -R -v $(local_dir)ni-libraries/src/include/FRC_FPGA_ChipObject/* $(local_dir)headers/
	cp -R -v $(local_dir)ni-libraries/src/include/FRC_NetworkCommunication/* $(local_dir)headers/
	cp -R -v $(local_dir)ni-libraries/src/include/visa/* $(local_dir)headers/

	cd $(local_dir)headers/hal/; sed -e '/#include \"hal\/SimDevice\.h\"/s/^/\/\//g' -i HAL.h

	python2 $(local_dir)get_frc_arm_gcc_header.py | xargs -I '{}' find '{}' -type d -name "gnu" | xargs -I '{}' cp -R '{}' $(local_dir)headers/
	python2 $(local_dir)get_frc_arm_gcc_header.py | xargs -I '{}' find '{}' -type d -name "sys" | xargs -I '{}' cp -R '{}' $(local_dir)headers/
	python2 $(local_dir)get_frc_arm_gcc_header.py | xargs -I '{}' find '{}' -type f -name "glob.h" | xargs dirname | xargs -I '{}' bash -c 'cp -R {}/*.h $(local_dir)headers/'
	python2 $(local_dir)get_frc_arm_gcc_header.py | xargs -I '{}' find '{}' -type f -name "glob.h" | xargs dirname | xargs -I '{}' cp -R '{}/bits' $(local_dir)headers/
	python2 $(local_dir)get_frc_arm_gcc_header.py | xargs -I '{}' find '{}' -type f -path "*/include/stddef.h" | xargs -I '{}' cp -R '{}' $(local_dir)headers/

a-bot_clean: local_dir := $(local_dir)
a-bot_clean:
	rm -rf $(local_dir)libs/*
	rm -rf $(local_dir)headers/*

wpilib_clean: local_dir := $(local_dir)
wpilib_clean: update_submod
	cd $(local_dir)allwpilib; ./gradlew clean

ni_clean: local_dir := $(local_dir)
ni_clean: update_submod
	cd $(local_dir)ni-libraries; ./gradlew clean

clean: a-bot_clean wpilib_clean ni_clean

all: clean cp_libs cp_headers