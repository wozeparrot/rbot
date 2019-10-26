local_dir := $(dir $(lastword $(MAKEFILE_LIST)))

.PHONY: wpilib_compile cp_libs update_submod a-bot_clean wpilib_clean

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

a-bot_clean: local_dir := $(local_dir)
a-bot_clean:
	rm -rf $(local_dir)libs/*
	rm -rf $(local_dir)headers/*

wpilib_clean: local_dir := $(local_dir)
wpilib_clean: update_submod
	cd $(local_dir)allwpilib; ./gradlew clean

clean: a-bot_clean
clean: wpilib_clean